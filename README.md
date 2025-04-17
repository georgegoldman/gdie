# Google Drive ID Extractor (gdie)

A simple GTK application to extract file IDs from Google Drive links.

---

## Features

- **Extract Google Drive file IDs**: Automatically extract the file ID from Google Drive URLs.
- **Simple GTK Interface**: Lightweight and easy-to-use graphical interface.

---

## Installation

### 1. 📦 Download

[➡️ Download GDie .deb Package (v0.1.0)](https://github.com/georgegoldman/gdie/releases/download/v1.0.2/gdie-deb.deb)

```bash
sudo dpkg -i gdie-deb.deb
```

### 2. **Install the Package**

Once the `.deb` package is downloaded, open a terminal and install it with the following command:

```bash
sudo dpkg -i gdie-deb.deb
```
If you encounter any dependency issues, you can fix them by running:
```bash
sudo apt-get install -f
```
### 3. Launch the Application
After installation, you can launch the application by either searching for "Google Drive ID Extractor" in your application menu or running it directly from the terminal:
```bash
gdie
```
## Usage

Once launched, simply paste a Google Drive URL into the input box, and the tool will extract the file ID and display it for you.

## Contributing
If you'd like to contribute, feel free to fork the project, make changes, and submit a pull request.

## License
This project is licensed under the MIT License – see the LICENSE file for details.

## Acknowledgments
- [Rust Programming Language](https://www.rust-lang.org/)
- [GTK4](https://www.gtk.org/)
