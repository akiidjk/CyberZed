# CyberZed Extension

CyberZed è un'estensione progettata per il text editor Zed, che fornisce una serie di comandi utili per la manipolazione di stringhe e per l'esecuzione di operazioni di codifica, hashing, generazione UUID e operazioni di XOR tra stringhe di bytes.

Questa estensione include i seguenti comandi:

- **Codifica e Decodifica**: Base64, Hex, URL
- **Hashing**: SHA256, SHA384, SHA512, MD5
- **Generazione UUID**
- **Operazioni XOR** tra due stringhe di bytes

## Funzionalità principali

### Comandi di Codifica

1. **Base64**
   - `encode`: Codifica una stringa in formato Base64.
   - `decode`: Decodifica una stringa Base64 in testo leggibile.

2. **Hex**
   - `encode`: Codifica una stringa in formato esadecimale.
   - `decode`: Decodifica una stringa esadecimale in testo leggibile.

3. **URL**
   - `encode`: Codifica una stringa in URL encoding.
   - `decode`: Decodifica una stringa URL encoding.

### Comandi di Hashing

1. **SHA**
   - `sha256`: Calcola l'hash SHA-256 di una stringa.
   - `sha384`: Calcola l'hash SHA-384 di una stringa.
   - `sha512`: Calcola l'hash SHA-512 di una stringa.

2. **MD5**
   - `md5`: Calcola l'hash MD5 di una stringa.

### Generazione UUID

- `uuid`: Genera un nuovo UUIDv4.

### XOR tra due stringhe di bytes

La funzione `xor_bytes` esegue l'operazione di XOR tra due array di bytes. Se le lunghezze degli array sono diverse, l'array più lungo viene troncato alla lunghezza dell'array più corto. L'operazione è limitata alla lunghezza della stringa più corta.

### Struttura del progetto

```plaintext
├── commands
│   ├── encoding.rs     # Gestisce i comandi di codifica (base64, hex, URL)
│   ├── hashing.rs      # Gestisce i comandi di hashing (SHA, MD5)
│   └── mod.rs          # Modulo principale che include tutti i comandi
├── cyberzed.rs         # Main file che registra l'estensione CyberZed
└── utils
    ├── uuid_gen.rs     # Generazione di UUID
    └── xor.rs          # Funzione XOR tra due array di bytes
```

### Comandi supportati

1. **Base64**
   - `encode`: Codifica una stringa in base64.
   - `decode`: Decodifica una stringa base64 in testo leggibile.
2. **Hex**
   - `encode`: Codifica una stringa in formato esadecimale.
   - `decode`: Decodifica una stringa esadecimale in testo leggibile.
3. **URL**
   - `encode`: Codifica una stringa in URL encoding.
   - `decode`: Decodifica una stringa URL encoding.
4. **SHA**
   - `sha256`, `sha384`, `sha512`: Calcola l'hash SHA corrispondente.
5. **MD5**
   - `md5`: Calcola l'hash MD5 di una stringa.
6. **UUID**
   - `uuid`: Genera un nuovo UUID.
7. **XOR**
   - `xor`: Esegue un'operazione di XOR tra due stringhe di bytes e restituisce il risultato in formato esadecimale.

### Esempio di utilizzo

#### XOR tra due stringhe

Supponiamo di voler eseguire un XOR tra due stringhe:

```bash
$ xor ["xor", "encode", "Hello", "World"]
```

Questo comando restituirà il risultato dell'operazione di XOR tra le stringhe "Hello" e "World" in formato esadecimale.

#### Codifica e Decodifica Base64

Per eseguire la codifica e la decodifica di una stringa in Base64:

```bash
$ base64 ["base64", "encode", "Hello World"]
$ base64 ["base64", "decode", "SGVsbG8gV29ybGQ="]
```

#### Calcolare un hash SHA

Per calcolare un hash SHA256, ad esempio:

```bash
$ sha ["sha", "256", "Hello World"]
```

Questo calcolerà l'hash SHA256 della stringa "Hello World".

### Installazione

...

### Licenza

Questo progetto è rilasciato sotto la [MIT License](LICENSE).
```
