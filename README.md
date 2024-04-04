# DailyChallengeNotifier

A simple Rust application that emails you the Leetcode daily challenge at midnight UTC.

![Demo](https://raw.githubusercontent.com/exception-raised/DailyChallengeNotifier/main/media/demo.png)

## Installation

Thanks to Docker, getting this project up and running is rather simple.

1. Clone this repository.
   ```bash
   git clone https://github.com/exception-raised/DailyChallengeNotifier.git
   ```

2. Rename the .env.example file to .env and fill out the variables accordingly.
3. Run the following command after navigating to the project directory.
   ```bash
     docker compose up -d
   ```

## Usage
Once the Docker container is up and running, the application will automatically send you the Leetcode daily challenge via email at midnight UTC.

## Troubleshooting
If you encounter any issues during installation or setup, please check the following:

- Ensure that your .env file is correctly configured with the required environment variables.
- Check the Docker logs for any error messages that may indicate issues with the application or configuration.


## Alternative Installation (Without Docker)

Don't wanna use Docker? That's fine!

1. Follow step 2 from the Docker instructions to configure the .env file.
2. Build the application using Cargo:
   ```bash
     cargo build --release
   ```
3. Run the program manually:
   ```bash
     ./target/release/daily_challenge_notifier
  ```

You will know the program is successfully up and running if you see Welcome to Leetcode Daily Challenge Notifier.

## Troubleshooting

If you encounter any issues during manual setup, please ensure that:

- You have Rust and Cargo installed on your system.
- The required dependencies specified in the Cargo.toml file are installed.
- Permissions are set correctly for running the application and accessing necessary resources.

## Contributing
Contributions are welcome! If you have any suggestions, feature requests, or bug reports, please feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
