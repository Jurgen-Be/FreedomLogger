# FreedomLogger

Een professionele, thread-veilige logging bibliotheek voor Rust met automatische rotatie, meerdere output formaten en foutbestendige werking.

## Functies

- **Meerdere log niveaus** met filtering (ERROR, WARNING, INFO, DEBUG, TRACE)
- **Verschillende output patronen** van basis tot JSON gestructureerde logging
- **Automatische log rotatie** gebaseerd op configureerbare bestandsgroottelimieten
- **Thread-veilige gelijktijdige logging** met interne synchronisatie
- **Foutbestendige werking** - interne fouten crashen nooit je applicatie
- **Minimale dependencies** - alleen chrono voor timestamps
- **Eenvoudige initialisatie** - setup met één functie-aanroep

## Snel aan de Slag

Voeg toe aan je `Cargo.toml`:
```toml
[dependencies]
freedom_logger = { git = "https://github.com/Jurgen-Be/FreedomLogger" }
```

Basis gebruik:
```rust
use FreedomLogger::{init, info, warning, error, Pattern};

fn main() {
    // Logger één keer initialiseren
    FreedomLogger::init(Pattern::Basic, "./logs", "mijnapp");
    
    // Overal in je applicatie loggen
    info("Applicatie gestart");
    warning("Dit is een waarschuwing");
    error("Er ging iets mis");
}
```

## Installatie

```bash
cargo add FreedomLogger --git https://github.com/Jurgen-Be/FreedomLogger
```

## Gebruik Voorbeelden

### Basis Initialisatie
```rust
use FreedomLogger::{init, Pattern};

// Logt alle niveaus, 10MB bestanden, 5 backups
init(Pattern::Basic, "/var/log/mijnapp", "applicatie");
```

### Met Log Niveau Filtering
```rust
use FreedomLogger::{init_with_level, Pattern, LogLevel};

// Log alleen WARNING en ERROR berichten
init_with_level(Pattern::Detailed, "./logs", "app", LogLevel::Warning);
```

### Aangepaste Rotatie Instellingen
```rust
use FreedomLogger::{init_with_rotation, Pattern, LogLevel};

// 50MB bestanden, bewaar 10 backups
init_with_rotation(
    Pattern::Json,
    "./logs", 
    "service",
    LogLevel::Info,
    50 * 1024 * 1024,  // 50MB
    10                 // 10 backup bestanden
);
```

### Logging Functies
```rust
use FreedomLogger::{error, warning, info, debug, trace};

error("Kritieke systeemfout");
warning("Gebruik van verouderde API gedetecteerd");  
info("Gebruikersauthenticatie succesvol");
debug("Verwerken van request payload");
trace("Functie calculate_metrics betreden");
```

## Output Patronen

### Basis Patroon
```
[2025-09-08 14:30:45] INFO: Gebruiker succesvol ingelogd
[2025-09-08 14:30:46] ERROR: Database verbinding mislukt
```

### Gedetailleerd Patroon
```
[2025-09-08 14:30:45] [main.rs:42] INFO: Gebruiker succesvol ingelogd
[2025-09-08 14:30:46] [db.rs:158] ERROR: Database verbinding mislukt
```

### JSON Patroon
```json
{"timestamp":"2025-09-08 14:30:45","level":"INFO","message":"Gebruiker succesvol ingelogd","file":"main.rs","line":42,"thread":"main"}
{"timestamp":"2025-09-08 14:30:46","level":"ERROR","message":"Database verbinding mislukt","file":"db.rs","line":158,"thread":"worker-1"}
```

## Log Rotatie

FreedomLogger roteert automatisch logbestanden wanneer ze de geconfigureerde grootte overschrijden:

- `app.log` (huidige logbestand)
- `app.1.log` (meest recente backup)
- `app.2.log` (oudere backup)
- `app.N.log` (oudste backup, wordt verwijderd wanneer limiet bereikt)

Standaard instellingen: 10MB maximale bestandsgrootte, 5 backup bestanden bewaard.

## Foutafhandeling

FreedomLogger is ontworpen om foutbestendig te zijn:

- **Crasht nooit** - Interne fouten worden netjes afgehandeld
- **Stille werking** - Logging fouten onderbreken je applicatie niet
- **Gescheiden error log** - Interne problemen gelogd naar `logger_errors.log`
- **Automatische fallbacks** - Ongeldige configuraties gebruiken veilige standaarden
- **Directory creatie** - Maakt log directories automatisch aan

## Thread Veiligheid

FreedomLogger is volledig thread-veilig:

```rust
use std::thread;
use FreedomLogger::{init, info, Pattern};

fn main() {
    init(Pattern::Basic, "./logs", "threaded_app");
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                info(&format!("Bericht van thread {}", i));
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Configuratie Opties

### Initialisatie Functies

| Functie | Log Niveau | Rotatie | Gebruik |
|---------|------------|---------|---------|
| `init()` | Alle niveaus | Standaard (10MB, 5 backups) | Ontwikkeling, testen |
| `init_with_level()` | Gefilterd | Standaard (10MB, 5 backups) | Productie met filtering |
| `init_with_rotation()` | Gefilterd | Aangepast | High-volume productie |

### Log Niveaus (Hiërarchisch)

- **ERROR** - Kritieke fouten, systeemfouten
- **WARNING** - Potentiële problemen, verouderd gebruik
- **INFO** - Algemene applicatie flow informatie
- **DEBUG** - Gedetailleerde debugging informatie
- **TRACE** - Zeer uitgebreide tracing informatie

### Patronen

- **Basic** - Eenvoudig timestamp, niveau, bericht formaat
- **Detailed** - Inclusief bron bestand en regel nummer
- **Extended** - Voegt thread informatie toe (gepland)
- **JSON** - Gestructureerde logging voor analyse tools
- **Custom** - Gebruiker-gedefinieerde format strings (gepland)

## Bestand Extensies

FreedomLogger gebruikt automatisch de juiste bestandsextensies:

- **Tekst patronen** (Basic, Detailed, Extended, Custom) → `.log` bestanden
- **JSON patroon** → `.json` bestanden

## Prestaties

- **Gebufferde I/O** - Gebruikt `BufWriter` voor optimale schrijfprestaties
- **Minimale allocaties** - Efficiënte string formatting en geheugengebruik
- **Thread synchronisatie** - Mutex-beschermde schrijfacties voorkomen data corruptie
- **Lazy initialisatie** - Logger componenten alleen gemaakt wanneer nodig

## Voorbeelden

Complete voorbeelden zijn beschikbaar in de `examples/` directory:

```bash
# Basis logging voorbeeld
cargo run --example basic_usage

# JSON gestructureerde logging
cargo run --example json_logging

# High-volume logging met rotatie
cargo run --example rotation_demo
```

## Vereisten

- **Rust** 1.70 of later
- **Dependencies**: chrono (timestamps), tempfile (alleen dev/testing)

## Bijdragen

Bijdragen zijn welkom! Zie [CONTRIBUTING.md](CONTRIBUTING.md) voor richtlijnen.

## Roadmap

### Versie 2.0 (Gepland)
- Database integratie voor directe log opslag
- Tijd-gebaseerde rotatie (dagelijks, wekelijks, maandelijks)
- Async logging voor high-performance applicaties
- Verbeterde caller locatie tracking
- Volledige custom patroon parsing

### Versie 1.x (Onderhoud)
- Bug fixes en prestatie verbeteringen
- Aanvullende output formaten
- Uitgebreide platform ondersteuning

## Licentie

Gelicenseerd onder:
- MIT License

## Changelog

Zie [CHANGELOG] voor gedetailleerde versie geschiedenis.

## Ondersteuning

- **Issues**: [GitHub Issues](https://github.com/yourusername/FreedomLogger/issues)
- **Discussies**: [GitHub Discussions](https://github.com/yourusername/FreedomLogger/discussions)
- **Documentatie**: [docs.rs/FreedomLogger](https://docs.rs/FreedomLogger)

---

*Gebouwd met Rust voor prestaties, veiligheid en betrouwbaarheid.*