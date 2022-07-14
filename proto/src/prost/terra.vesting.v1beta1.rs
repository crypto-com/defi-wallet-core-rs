/// LazyGradedVestingAccount implements the LazyGradedVestingAccount interface. It vests all
/// coins according to a predefined schedule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LazyGradedVestingAccount {
    #[prost(message, optional, tag="1")]
    pub base_vesting_account: ::core::option::Option<super::super::super::cosmos::vesting::v1beta1::BaseVestingAccount>,
    #[prost(message, repeated, tag="2")]
    pub vesting_schedules: ::prost::alloc::vec::Vec<VestingSchedule>,
}
/// Schedule - represent single schedule data for a vesting schedule
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    #[prost(int64, tag="1")]
    pub start_time: i64,
    #[prost(int64, tag="2")]
    pub end_time: i64,
    #[prost(string, tag="3")]
    pub ratio: ::prost::alloc::string::String,
}
/// VestingSchedule defines vesting schedule for a denom
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VestingSchedule {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub schedules: ::prost::alloc::vec::Vec<Schedule>,
}
