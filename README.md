<div align="center">
  <pre>
███████╗██╗   ██╗███████╗
██╔════╝██║   ██║██╔════╝
█████╗  ██║   ██║█████╗  
██╔══╝  ╚██╗ ██╔╝██╔══╝  
███████╗ ╚████╔╝ ███████╗
╚══════╝  ╚═══╝  ╚══════╝
  </pre>
</div>

EVE is a secure, offline Diceware passphrase generator written in Rust.

Rather than generating passwords you can't remember, EVE produces human-readable passphrases using a cryptographically secure RNG, making them both memorable and mathematically strong.

---
## Features
- **True entropy** - uses a CSPRNG seeded directly from the operating system
- **Curated French wordlist** - 55'555 words sourced from [ArthurPons](https://github.com/ArthurPons/diceware-fr-alt/blob/master/diceware-fr-alt.txt)
- **Configurable length:** Choose how many words compose your passphrase at runtime
- **Leaves memory clean:** properly wipes all sensitive data in memory before exiting
- **Fully offline:** no network calls, no telemetry, no external services

---
## Installation

### From source

```bash
git clone https://github.com/b0cal/eve
cd eve
cargo build --release
```

The compiled binary will be at `target/release/eve`

### Prerequisites

- Rust stable (1.70 or later) - install via [rustup](https://rustup.rs) 
- The wordlist file `wordlist-fr.txt` must be present in the working directory, or its path passed via the  `WORDLIST_PATH` environment variable (see [Usage](#usage))

---
## Usage

```bash
./eve
```

You will be prompted to enter the number of words for your passphrase:

```
> Combien de mots doivent composer la passphrase ? 8
La passphrase est: reproche-promo-courtisan-cloporte-argent-junior-incendiaire-parthe
```
---
## How it works

EVE implements the [Diceware](https://theworld.com/~reinhold/dicewarefaq.html) method, originally designed around physical dice, adapted here with a larger wordlist and a software CSPRNG:

### The generation process

1. Five dice are rolled to produce a 5-digit number (e.g. `12634`)
2. That number is looked up in a wordlist of 55'555 words
3. Steps 1-2 are repeated for the requested number of words
4. Words are joined with dashes to form the final passphrase

### Entropy


Each word drawn from a 55,555-word list contributes **~15.76 bits of entropy** (log₂(55,555)), compared to 12.2 bits with the classic, 7,776-word Diceware list. The table below shows the total entropy and a rough security estimate for each word count:

| Words | Entropy | Security estimate |
|---|---|---|
| 5 | ~78.8 bits | Strong for most personal use |
| 6 | ~94.6 bits | Resistant to well-funded attackers |
| 7 | ~110.3 bits | Secure beyond 2030 | 
| 8 | ~126.1 bits | Secure beyond 2050 |

For more context on passphrase strength and thread modelling see [NIST SP 800-64B, Appendix A](https://pages.nist.gov/800-63-4/sp800-63b/passwords/)

### A note on modulo bias

The wordlist contains 55,555 words - not a power of 2. When mapping a uniformly distributed RNG output to a wordlist that doesn't divide the RNG range evenly, some words are marginally more likely than others. This is called **modulo bias**.

In practie, the bias per word is on the order of 10⁻⁶ - negligible for passphrase generation. The entropy gain from the larger wordlist (~2.84 extra bits per word over classic Diceware) far outweighs this theoretical impurity. A future release may implement rejection sampling to eliminate the bias entirely.

---
## Documentation

Generate and open the full API documentation locally
```bash
# Just EVE's own documentation
cargo doc --open --no-deps

# EVE + all dependencies
cargo doc --open
```

---
## Project structure

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── lib.rs    ← core logic: wordlist loading, dice rolling, passphrase generation
│   └── main.rs   ← CLI entrypoint: user interaction and error handling
├── THIRD_PARTY_LICENSE.md
└── wordlist-fr.txt
```
---
## Contributing

EVE is MIT licensed - you are free to fork it, adapt it, and use it in any project, including commercial ones. No contribution process is set up at this time.

If you find a bug or have a suggestion, feel free to open an issu

---
## Security considerations


- **RNG:** EVE uses `StdRng` seeded from `SysRng`, which draws entropy from the operating system (`/dev/urandom` on Linux). This is cryptographically appropriate for passphrase generation
- **Wordlist:** Loaded once at startup into memory and never written to disk
- **Network:** EVE makes no network calls whatsoever
- **Modulo bias:** See the [note on modulo bias](#a-note-on-modulo-bias) in the How it works section
- **Threat model:** EVE generates strong passphrases but does not protect against keystroke logging, phishing or shoulder surfing. Physical and operational security remain your responsibility
- **Auditability:** The codebase is intentionally small. If you have specific threat model requirements, the full logic lives in `src/lib.rs`

---
## License

MIT - see [LICENSE](./LICENSE) for details.

---
## Acknowledgements

The French Diceware wordlist (`wordlist-fr.txt`) is sourced from [diceware-fr-alt](https://github.com/ArthurPons/diceware-fr-alt) by ArthurPons, used under the MIT license.

