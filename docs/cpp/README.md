# Defi Wallet Core Cpp Documents Generation Guide
## Doxygen
### Installation

    brew install doxygen


### Generate a `Doxyfile` file

    doxygen -g

### Edit `Doxyfile`

    PROJECT_NAME           = "Defi Wallet Core Cpp"
    OUTPUT_DIRECTORY       = doxygen
    JAVADOC_AUTOBRIEF      = YES
    EXTRACT_ALL            = YES
    INPUT                  = ../../example/cpp-example/rust ../example/cpp-example/defi-wallet-core-cpp/src
    FILE_PATTERNS          = *.h
    EXCLUDE_SYMBOLS        = org$defi_wallet_core$cxxbridge1$* \
                            cxxbridge1$box$org$defi_wallet_core$* \
                            std
    GENERATE_HTML          = YES
    GENERATE_LATEX         = NO
    GENERATE_XML           = YES
    ENABLE_PREPROCESSING   = NO

### Generate the `doxygen/html` and `doxygen/xml` folder

    doxygen

## sphnix
### Installation

    pip install -U sphinx
    pip install sphinx-book-theme
    pip install sphinx_rtd_theme
    pip install breathe
    pip install exhale

### Setup a sphnix project with `sphinx-quickstart`

    mkdir sphinx
    cd sphinx
    sphinx-quickstart

Then follow the prompts and input the necessary project information

### Edit `sphinx/conf.py`

#### General configuration

Add breathe, exhale extensions and configure them

```
# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    "breathe",
    "exhale",
]

# Breathe Configuration
breathe_default_project = "Defi Wallet Core Cpp"
breathe_projects = {"Defi Wallet Core Cpp": "../doxygen/xml"}
# Setup the exhale extension
exhale_args = {
    # These arguments are required
    "containmentFolder":     "./api",
    "rootFileName":          "library_root.rst",
    "rootFileTitle":         "Library API",
    "doxygenStripFromPath":  "..",
    # Suggested optional arguments
    "createTreeView":        True,
    # TIP: if using the sphinx-bootstrap-theme, you need
    # "treeViewIsBootstrap": True,
}
```

### Edit `sphinx/index.rst`

Add a new section

```
Docs
====

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   api/library_root

```


### Make

    make html

The html is in `sphinx/_build/html/index.html`

## Doxygen2
`Doxygen2` can convert `Doxygen` XML to Markdown, and we further serve the Markdown to other custom generators, for example `GitBook` or `mdbook`.

### Installation
- Install [Doxygen2](https://github.com/matusnovak/doxybook2)
- Install [GitBook](https://github.com/GitbookIO/gitbook)
- Install [mdbook](https://rust-lang.github.io/mdBook/guide/installation.html)
- Create a `config.json` and a SUMMARY template `SUMMARY.md.tmpl`, refer to https://github.com/matusnovak/doxybook2/tree/master/example/gitbook
- Generate `GitBook` with `doxybook2`

        doxybook2 \
            --input doxygen/xml \
            --output gitbook/src \
            --config config.json \
            --summary-input SUMMARY.md.tmpl \
            --summary-output gitbook/src/SUMMARY.md

- Serve with `GitBook`

        cd gitbook/src && gitbook serve

- Generate `mdbook` with `doxybook2`

        doxybook2 \
            --input doxygen/xml \
            --output mdbook/src \
            --config config.json \
            --summary-input SUMMARY.md.tmpl \
            --summary-output mdbook/src/SUMMARY.md

- Serve with `mdbook`

        cd mdbook && mdbook serve --open
