# Lorem

A simple terminal app for generating lorem ipsum text.

## Usage

Generate 8 words of Lorem Ipsum text

```
 $ lorem 8
 Quaedam et quasi involuta aperiri, altera prompta et.
```

Generate 8 paragraphs of Lorem Ipsum text

```
 $ lorem -p 8
 Se et Consentinis et Siculis scribere. F...
```

Export the seed for reuse. This will be output to stderr

```
 $ lorem -d 8
 1595535435300688293
 Multa venustate et omni sale idem Lucilius, apud.
```

Reuse a seed

```
 $ lorem -s 1595535435300688293 8
 Multa venustate et omni sale idem Lucilius, apud.
```

## Building

```
 $ cargo build --release
```

## Installing

From crates.io
```
 $ cargo install lorem
```

From source
```
 $ cargo install --path .
```