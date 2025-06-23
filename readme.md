# How to Run and Build This App on a VPS

## Prerequisites

- Rust (latest stable) installed  
    [Install Rust](https://www.rust-lang.org/tools/install)
- `git` installed
- Access to your VPS (SSH)

## Steps

1. **Clone the Repository**

     ```sh
     git clone https://github.com/your-username/rocket_reysofts_courses.git
     cd rocket_reysofts_courses
     ```

2. **Build the Application**

     ```sh
     cargo build --release
     ```

3. **Run the Application**

     ```sh
     cargo run --release
     ```

     Or run the compiled binary:

     ```sh
     ./target/release/rocket_reysofts_courses
     ```

4. **Access the App**

     By default, the app runs on `http://localhost:8000`.  
     To access it remotely, ensure your VPS firewall allows traffic on the app's port.

## Additional Notes

- Edit `.env` or configuration files as needed.
- For production, consider using a process manager like `systemd` or `pm2` to keep the app running.


com este projeto eu vi ue preciso ter o ruust para o back end e o solidjs/vite ou qualquer outro nodejs front para o front end, eh melhor, ainda mais com o jwt
