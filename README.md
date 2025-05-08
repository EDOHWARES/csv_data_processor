# CSV Data Filter CLI (Rust)

This is a simple command-line tool built in Rust that reads data from a CSV file and filters it based on a specified column and value.

---

## 📁 Project Structure

```bash
csv_data_processor/
├── data.csv # CSV file containing data
├── src/
│ ├── main.rs # Entry point of the application
│ ├── person.rs # Struct definition for CSV records
│ └── utility.rs # Logic to read and filter CSV
├── Cargo.toml # Rust project metadata and dependencies
└── README.md # Project overview and instructions
```


---

## 📦 Requirements

- Rust (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust package manager)

---

## 🚀 How to Run

### 1. Clone the repo:

```bash
git clone https://github.com/EDOHWARES/csv_data_processor
cd csv_data_processor
```

### 2. Add your CSV file (or use the existing data.csv):

```bash
name,age,city
Alice,30,Lagos
Bob,25,Abuja
Charlie,30,Kano
```


## ⚙️ Usage:

### 1. Basic Filtering (uses default data.csv);

```bash
cargo run name Alice
```

### 2. Custom Input File:

```bash
cargo run city Kano --file students.csv
```

### 3. Save Filtered Results to a New File:

```bash
cargo run city Lagos --output lagos.csv
```

### 4. Combine Input and Output:

```bash
cargo run age 25 --file people.csv --output age25.csv
```

---

### 🧑‍💻 Author

```bash
@EDOHWARES
```