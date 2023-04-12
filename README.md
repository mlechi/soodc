# soodc
This is the (check)Summer Of Our Disk Content

Ok, I know checksums are not the same as hashes, but the pun is too good.
This is a very simple program that automates the process of comparing a downloaded file to the associated hash from the source of the file.

Could I have done this in bash? Yes! Should I have? Probably... But Rust is cool and I want to start doing everything I can in Rust.

Right now it only does Sha256, but I will make this work with all the other algorithms in the sha2 crate at least.
