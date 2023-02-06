# A-Maze-Eng MARV Test Kit

This is the main source code repository for the "A-Maze-Eng MARV Test Kit".

The purpose of this software is to allow Computer, Electronic, and Electrical Engineering students at the University of Pretoria to test the subsystems that will form their 3rd year autonomous maze robots, to be built for their course in Systems Engineering (EPR 320).

## List of Abbreviations and Acronyms

- QTP: Quality Test Protocol
- SNC: State and Navigation Control Sybsystem
- SS: Sensor Subsystem
- MDPS: Motor Driver and Power supply Subsystem
- NAVCON: Navigation Control (of SNC)

For more information see the [Practical Guide].

[practical guide]: docs/Practical_Guide_to_an_AMazeENG_MARV_2022v3.pdf

## How to use

### Disclaimer

As of 6 February 2023, this software cannot yet test physical hardware, however you can clone this repository and run the code to see how the emulations work for the implemented QTPs.

### Running the project

#### 1. Install Rust

See [Rust Installation]

[rust installation]: https://doc.rust-lang.org/book/ch01-01-installation.html

#### 2. Clone the repo with `git`

Enter the following terminal command

```sh
git clone https://github.com/Reinhardtvbm/A-Maze-Eng_MARV_Test_Kit.git
```

If you do not have git installed, see [Git Download]

[git download]: https://git-scm.com/downloads

#### 3. Run the code

After cloning the repo, enter to follwing two commands

```sh
cd A-Maze-Eng_MARV_Test_Kit/
cargo run --release
```

### Using the App

The application will open on the following screen

<img src = "docs\images\2023-02-06 11_51_35-A-Maze-Eng-MARV Test Kit.png">
