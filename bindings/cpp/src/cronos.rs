use crate::ffi::CronosTransactionReceiptRaw;
use anyhow::{anyhow, Result};
use ethers::prelude::TransactionReceipt;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::{runtime::Runtime, sync::mpsc};
#[derive(Debug, Default)]
pub struct CronosTxMessage {
    txreceipt: CronosTransactionReceiptRaw,
}
// request channel
#[derive(Debug, Default)]
pub struct CronosTxRequest {
    signedtx: Vec<u8>,
    rpcurl: String,
    jobid: String,
}

// handle tx managing
pub struct CronosTxManager {
    runtime: Option<Runtime>,
    receive_receipt: Option<mpsc::UnboundedReceiver<CronosTxMessage>>,
    send_reqeust: Option<mpsc::UnboundedSender<CronosTxRequest>>,
}

impl CronosTxManager {
    pub fn new() -> Result<CronosTxManager> {
        Ok(CronosTxManager {
            runtime: Some(Runtime::new().expect("Failed to create runtime")),
            receive_receipt: None,
            send_reqeust: None,
        })
    }

    // start processing tx
    pub fn start_working(&mut self) -> bool {
        // create request channel
        let (send, mut receive) = mpsc::unbounded_channel::<CronosTxRequest>();
        self.send_reqeust = Some(send);

        // create receipt channel
        let (sendreceipt, receivereceipt) = mpsc::unbounded_channel::<CronosTxMessage>();
        self.receive_receipt = Some(receivereceipt);

        match &self.runtime {
            Some(v) => {
                v.spawn({
                    async move {
                        loop {
                            // stopped when runtime is dropped
                            // read from rx
                            let cronos_tx_request = receive.recv().await.unwrap();
                            let jobid = cronos_tx_request.jobid.clone();
                            // received tx-receipt
                            let tx_receipt = CronosTxManager::broadcast_tx(
                                cronos_tx_request.signedtx,
                                cronos_tx_request.rpcurl,
                            )
                            .await;

                            match tx_receipt {
                                Ok(mut receipt) => {
                                    receipt.success = true;
                                    receipt.jobid = jobid;
                                    receipt.message = "".into();
                                    let cronos_tx_message = CronosTxMessage { txreceipt: receipt };
                                    sendreceipt.send(cronos_tx_message).unwrap();
                                }
                                Err(e) => {
                                    let receipt = CronosTransactionReceiptRaw {
                                        jobid,
                                        success: false,
                                        message: e.to_string(),
                                        ..Default::default()
                                    };
                                    let tx_message = CronosTxMessage { txreceipt: receipt };
                                    sendreceipt.send(tx_message).unwrap();
                                }
                            } // end of match
                        } // end of loop
                    } // end of async
                }); //end of spawn
                true
            }
            None => false,
        }
    }

    // stop current working, blocking operation will be stopped
    pub fn stop_working(&mut self) -> bool {
        match &self.runtime {
            Some(_) => {
                self.receive_receipt = None;
                self.send_reqeust = None;
                self.runtime = None;
                true
            }
            None => false,
        } // end of match
    }

    // broadcast tx to cronos, aysnc operation
    async fn broadcast_tx(
        raw_tx: Vec<u8>,
        web3api_url: String,
    ) -> Result<CronosTransactionReceiptRaw> {
        let result: TransactionReceipt =
            defi_wallet_core_common::broadcast_eth_signed_raw_tx(raw_tx, &web3api_url).await?;
        let cronos_tx_receipt: CronosTransactionReceiptRaw = result.into();
        Ok(cronos_tx_receipt)
    }

    // broadcast tx to cronos in async way, result can be polled by get_broadcast_tx_async
    pub fn broadcast_eth_signed_raw_tx_async(
        self: &'static mut CronosTxManager,
        raw_tx: Vec<u8>,
        web3api_url: String,
        jobid: String,
    ) -> Result<()> {
        let cronos_tx_request = CronosTxRequest {
            signedtx: raw_tx,
            rpcurl: web3api_url,
            jobid,
        };
        let send_channel = self.send_reqeust.clone();

        match &self.runtime {
            Some(target_runtime) => {
                target_runtime.spawn(async move {
                    match &send_channel {
                        Some(tx_request) => tx_request
                            .send(cronos_tx_request)
                            .map_err(|send_error| anyhow!("{}", send_error)),
                        None => Err(anyhow!("send channel is not initialized")),
                    }
                });
                Ok(())
            }
            None => Err(anyhow!("runtime is not initialized")),
        } // end of match
    }

    // polling broadcast tx result , no waiting
    pub fn get_broadcast_tx_async(
        self: &'static mut CronosTxManager,
    ) -> Result<CronosTransactionReceiptRaw> {
        match &mut self.receive_receipt {
            Some(received_tx_message) => match received_tx_message.try_recv() {
                Ok(tx_message) => Ok(tx_message.txreceipt),
                Err(e) => Err(anyhow!("{}", e)),
            },
            None => {
                return Err(anyhow!("no data",));
            }
        } // end of match
    } // end of function

    async fn get_broadcast_tx_blocking_do(
        receiver: &mut UnboundedReceiver<CronosTxMessage>,
    ) -> Result<CronosTransactionReceiptRaw> {
        let tx_message = receiver.recv().await.ok_or_else(|| anyhow!("no data"))?;
        Ok(tx_message.txreceipt)
    }

    // get broadcast tx result with blocking
    // blocking is stopped when CronosTxManager is destroyed
    pub fn get_broadcast_tx_blocking(
        self: &'static mut CronosTxManager,
    ) -> Result<CronosTransactionReceiptRaw> {
        match &mut self.runtime {
            Some(runtime) => match &mut self.receive_receipt {
                Some(tx_message) => {
                    runtime.block_on(CronosTxManager::get_broadcast_tx_blocking_do(tx_message))
                }
                None => Err(anyhow!("no data")),
            },
            None => Err(anyhow!("no runtime",)),
        } // end of match
    } // end of function
}
