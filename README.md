# random_nanoka
## The worst random number generator in the world, nanodesu.

## How does it work?
### There's a vec of anime characters who say nanoka a lot.
```rust
        let nanoka_characters = vec![
            "Hachikuji Mayoi",
            "Araragi Koyomi",
            "Senjougahara Hitagi",
            "Kanbaru Suruga",
            "Hanekawa Tsubasa",
            "Oshino Shinobu",
        ];
```
### And it gets the length of their names.
```rust
nanoka_characters.len()
```
### And uses that for generating a number.
```rust
    fn next(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(self.nanoka_characters.len() as u64);
        (self.seed)
    }

    fn next_in_range(&mut self, min: u64, max: u64) -> u64 {
        min + self.next() % (max - min + 1)
    }
```
### Shinobu Oshino
![Shinobu Oshino](https://github.com/Smallsan/random_nanoka/assets/126304322/6b8f5a3d-9f4f-43e3-93bc-b623d002214c)
