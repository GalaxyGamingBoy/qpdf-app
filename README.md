# PDF Visual Toolkit based on QPDF

![GitLab Issues](https://img.shields.io/gitlab/issues/open/mitsiosm%2Fqpdf-app)
![GitLab Merge Requests](https://img.shields.io/gitlab/merge-requests/open/mitsiosm%2Fqpdf-app)
![GitLab License](https://img.shields.io/gitlab/license/mitsiosm%2Fqpdf-app)
![GitLab Last Commit](https://img.shields.io/gitlab/last-commit/mitsiosm%2Fqpdf-app)
  
> [!note]
> This is an independent project and is not endorsed or affiliated by [qpdf](https://github.com/qpdf/qpdf)

PDF Visual Toolkit based on [QPDF](https://github.com/qpdf/qpdf). Ever had to merge a pdf, reorganize it's pages or keep only a range of them? The PDF Visual Toolkit is a GUI application made in Rust with Tauri and QPDF that aims to solve those problems in a intuitive and FOSS application, without the need of all the other online tools. All of the PDF handling gets done by QPDF through the libqpdf-rs wrapper for rust made by me for this purpose.

## Features
 
- PDF Merging
- PDF Splitting
- PDF Reordering/Reorganizing
- WIP

## Tech Stack

- Vue
- Rust
- QPDF
- Tauri

## Feedback / Support
If you have any feedback or need support
+ Email me, [here](mailto:contact-project+mitsiosm-qpdf-app-71506270-issue-@incoming.gitlab.com)
+ Open a [Gitlab](https://gitlab.com/mitsiosm/qpdf-app) issue
+ Open a [Github](https://github.com/GalaxyGamingBoy/qpdf-app) issue (not preferred)

## Acknowledgements

 - [QPDF Library](https://github.com/qpdf/qpdf)

## Authors

- [@MitsiosM](https//gitlab.com/mitsiosm/)


## Run Dev Locally

Clone the project

```bash
  git clone https://gitlab.com/mitsiosm/qpdf-app.git
```

Go to the project directory

```bash
  cd qpdf-app
```

Initalize the Submodules

```bash
    git submodule init 
    git submodule update
```

Run the development server

```bash
  pnpm tauri dev
```
### Environment Variables

To run this project, you will need to add to add the appropriate app settings or user secrets.

- `VITE_OUTSIDE_TAURI`: Set to true to use placeholder values when running outside of the tauri environment (e.g. only web dev server)