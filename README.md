
# Yandex Mail Webapp Client

This project is a webapp client for Yandex Mail, built using the beta Tauri app framework. It includes features for multi-language support and session management.

## Features

- **Multi-language Support**: Translate the application interface to French, German, and Spanish using JSON translation files.
- **Session Management**: Store and manage multiple user sessions through a user-friendly menu.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (version 12 or higher)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Clone the Repository

```bash
git clone https://github.com/your-username/yandex-mail-webapp-client.git
cd yandex-mail-webapp-client
```

### Install Dependencies

```bash
npm install
```

### Run the Application

```bash
npm run tauri dev
```

## Releases

You can download the latest release of the Yandex Mail Webapp Client from the [Releases](https://github.com/kmoz000/Yandex-Email-app/releases) page.

### Downloading and Installing

1. Visit the [Releases](https://github.com/kmoz000/Yandex-Email-app/releases) page.
2. Download the appropriate version for your operating system.
3. Follow the installation instructions provided in the release notes.

## Configuration

### Language Translation

Translation files are located in the `src/translations` directory. Each language has its own JSON file (e.g., `en.json`, `fr.json`, `de.json`, `es.json`).

To add or update translations, edit the corresponding JSON file.

### Session Management

Sessions are stored in the `src/sessions` directory. Each session is saved as a JSON file containing user credentials and settings.

## Usage

### Switching Languages

To switch the application language, use the language selector menu in the application. The translations will be loaded from the corresponding JSON files.

### Managing Sessions

To store multiple sessions, use the session management menu. You can add, edit, or delete sessions, and switch between them easily.

## Menu Guide

### Language Selector

- **Location**: Main Menu > Settings > Language
- **Options**: English, French, German, Spanish
- **Function**: Select your preferred language to translate the application interface.

### Session Management

- **Location**: Main Menu > Sessions
- **Options**:
  - **Add Session**: Save a new user session by entering credentials and settings.
  - **Edit Session**: Modify the details of an existing session.
  - **Delete Session**: Remove a session from the list.
  - **Switch Session**: Activate a different session from the saved sessions list.

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Feel free to customize this README further to fit your specific needs and project details.