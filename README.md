The LS810-BS device is a BLE devices that performs medical measurements and then advertises itself a period of time then turns itself down. This code helps measuring the length of that period of time, it can be used for other devices if you change the uuid service that you are looking for. It has an error of 5 seconds per measurement.

# Results
For five consecutive measurements for the LS810-BS device, I have got the following results:

➜  measure-ls810-bs-advertisement-time git:(master) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/measure-ls810-bs-advertisement-time`
Scanning for devices...
Target device with service 0x7809 started advertising.
Target device with service 0x7809 stopped advertising.
Advertising duration: 65.931146041s

➜  measure-ls810-bs-advertisement-time git:(master) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/measure-ls810-bs-advertisement-time`
Scanning for devices...
Target device with service 0x7809 started advertising.
Target device with service 0x7809 stopped advertising.
Advertising duration: 65.965146041s
➜  measure-ls810-bs-advertisement-time git:(master) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/measure-ls810-bs-advertisement-time`
Scanning for devices...
Target device with service 0x7809 started advertising.
Target device with service 0x7809 stopped advertising.
Advertising duration: 65.959748708s
➜  measure-ls810-bs-advertisement-time git:(master) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/measure-ls810-bs-advertisement-time`
Scanning for devices...
Target device with service 0x7809 started advertising.
Target device with service 0x7809 stopped advertising.
Advertising duration: 60.886326416s
➜  measure-ls810-bs-advertisement-time git:(master) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/measure-ls810-bs-advertisement-time`
Scanning for devices...
Target device with service 0x7809 started advertising.
Target device with service 0x7809 stopped advertising.
Advertising duration: 65.961109375s

# Conclusion

The LS810-BS device advertises for about 60 +/- 5 seconds. 