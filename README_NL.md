# FreedomLogger 🦀

Een professionele, thread-veilige logging bibliotheek voor Rust met automatische rotatie, meerdere output formaten en foutbestendige werking.

## ✨ Functies

- **Meerdere log niveaus** met filtering (ERROR, WARNING, INFO, DEBUG, TRACE)
- **Flexibele logging API** - Zowel eenvoudige functies als geformatteerde macro's
- **Verschillende output patronen** van basic tot JSON gestructureerde logging
- **Automatische log rotatie** gebaseerd op configureerbare bestandsgrootte limieten
- **Thread-veilige gelijktijdige logging** met interne synchronisatie
- **Foutbestendige werking** - interne fouten laten je applicatie nooit crashen
- **Minimale afhankelijkheden** - alleen chrono voor timestamps
- **Eenvoudige initialisatie** - setup met één functie aanroep

## 🚀 Snel Starten

### Installatie

Voeg toe aan je `Cargo.toml`:
```toml
[dependencies]
freedom_logger = "1.1.0"
```

Of gebruik cargo:
```bash
cargo add freedom_logger
```

### Basis gebruik:
```rust
use freedom_logger::{log_init, log_info, log_warning, log_error, Pattern};

fn main() {
    // Initialiseer logger één keer
    log_init(Pattern::Basic, "./logs", "mijnapp");
    
    // Log overal in je applicatie
    log_info("Applicatie gestart");
    log_warning("Dit is een waarschuwing");
    log_error("Er is iets fout gegaan");
}
```

## 🆕 Nieuw in v1.1.0: Geformatteerde Logging Macro's

FreedomLogger ondersteunt nu zowel eenvoudige functies als krachtige formatterings macro's:

### Eenvoudige Functies (Origineel)
```rust
use freedom_logger::{log_error, log_warning, log_info, log_debug, log_trace};

log_error("Kritieke systeemfout");
log_warning("Verouderde API gebruik gedetecteerd");
log_info("Gebruiker authenticatie succesvol");
log_debug("Verwerken van request payload");
log_trace("Functie calculate_metrics wordt aangeroepen");
```

### Nieuwe Formatterings Macro's
```rust
use freedom_logger::{log_error, log_warning, log_info, log_debug, log_trace};

// Ondersteuning voor geformatteerde strings met automatische type behandeling
log_info!("Gebruiker {} succesvol ingelogd", gebruikersnaam);
log_debug!("Database pad: {:?}", database_pad);  // Werkt met elk Debug type!
log_error!("Verbinding mislukt naar {}: {}", host, fout_bericht);
log_warning!("Verwerken van {} items in batch {}", item_aantal, batch_id);

// Complexe types werken automatisch
let config = MijnConfig { host: "localhost", port: 5432 };
log_debug!("Server configuratie: {:?}", config);

// Meerdere format specifiers
log_info!("Gebruiker {} van {} ingelogd om {}", user_id, ip_adres, tijdstempel);
```

### Waarom de Macro's Gebruiken?

**Voor v1.1.0** (zou compilatie fouten veroorzaken):
```rust
let database_pad = PathBuf::from("/var/lib/app.db");
log_debug("Database pad: {:?}", database_pad); // ❌ Fout!
```

**Na v1.1.0** (werkt perfect):
```rust
let database_pad = PathBuf::from("/var/lib/app.db");
log_debug!("Database pad: {:?}", database_pad); // ✅ Perfect!
```

## 📋 Initialisatie Opties

### Basis Setup
```rust
use freedom_logger::{log_init, Pattern};

// Logt alle niveaus, 10MB bestanden, 5 backups
log_init(Pattern::Basic, "/var/log/mijnapp", "applicatie");
```

### Met Log Niveau Filtering
```rust
use freedom_logger::{log_init_with_level, Pattern, LogLevel};

// Log alleen WARNING en ERROR berichten
log_init_with_level(Pattern::Detailed, "./logs", "app", LogLevel::Warning);
```

### Volledige Configuratie
```rust
use freedom_logger::{log_init_with_rotation, Pattern, LogLevel};

// 50MB bestanden, bewaar 10 backups
log_init_with_rotation(
    Pattern::Json,
    "./logs",
    "service",
    LogLevel::Info,
    50 * 1024 * 1024, // 50MB
    10 // 10 backup bestanden
);
```

## 📝 Output Formaten

### Basic Patroon
```
[2025-09-09 14:30:45] INFO: Gebruiker succesvol ingelogd
[2025-09-09 14:30:46] ERROR: Database verbinding mislukt
```

### Gedetailleerd Patroon (met bron locatie)
```
[2025-09-09 14:30:45] [main.rs:42] INFO: Gebruiker succesvol ingelogd
[2025-09-09 14:30:46] [db.rs:158] ERROR: Database verbinding mislukt
```

### JSON Patroon (gestructureerde logging)
```json
{"timestamp":"2025-09-09 14:30:45","level":"INFO","message":"Gebruiker succesvol ingelogd","file":"main.rs","line":42,"thread":"main"}
{"timestamp":"2025-09-09 14:30:46","level":"ERROR","message":"Database verbinding mislukt","file":"db.rs","line":158,"thread":"worker-1"}
```

## 🔄 Automatische Log Rotatie

FreedomLogger roteert automatisch log bestanden wanneer ze de geconfigureerde grootte overschrijden:

```
app.log      (huidig log bestand)
app.1.log    (meest recente backup)
app.2.log    (oudere backup)
app.N.log    (oudste backup, wordt verwijderd bij limiet)
```

**Standaard instellingen:** 10MB max bestandsgrootte, 5 backup bestanden bewaard.

## 🛡️ Foutbestendige Werking

FreedomLogger is ontworpen om foutbestendig te zijn:

- **Nooit panic** - Interne fouten worden netjes behandeld
- **Stille werking** - Logging fouten onderbreken je applicatie niet
- **Apart fout log** - Interne problemen worden gelogd in `logger_errors.log`
- **Automatische fallbacks** - Ongeldige configuraties gebruiken veilige standaarden
- **Directory creatie** - Creëert automatisch log directories

## 🧵 Thread Veiligheid

FreedomLogger is volledig thread-veilig:

```rust
use std::thread;
use freedom_logger::{log_init, Pattern};

fn main() {
    log_init(Pattern::Basic, "./logs", "threaded_app");
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                // Beide stijlen werken in threads
                log_info!("Bericht van thread {}", i);
                log_debug!("Thread {} verwerkt data: {:?}", i, some_data);
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## 📊 Configuratie Snelle Referentie

| Functie | Log Niveau | Rotatie | Gebruik |
|---------|------------|---------|---------|
| `log_init()` | Alle niveaus | Standaard (10MB, 5 backups) | Ontwikkeling, testen |
| `log_init_with_level()` | Gefilterd | Standaard (10MB, 5 backups) | Productie met filtering |
| `log_init_with_rotation()` | Gefilterd | Aangepast | Hoog-volume productie |

## 📈 Log Niveaus

- **ERROR** - Kritieke fouten, systeem fouten
- **WARNING** - Potentiële problemen, verouderd gebruik
- **INFO** - Algemene applicatie flow informatie
- **DEBUG** - Gedetailleerde debugging informatie
- **TRACE** - Zeer uitgebreide tracing informatie

## 🎨 Beschikbare Patronen

- **Basic** - Eenvoudig timestamp, niveau, bericht formaat
- **Detailed** - Inclusief bron bestand en regel nummer
- **Extended** - Voegt thread informatie toe (gepland)
- **JSON** - Gestructureerde logging voor analyse tools
- **Custom** - Gebruiker-gedefinieerde format strings (gepland)

## 📁 Bestand Extensies

FreedomLogger gebruikt automatisch de juiste bestand extensies:
- Tekst patronen (Basic, Detailed, Extended, Custom) → `.log` bestanden
- JSON patroon → `.json` bestanden

## ⚡ Prestaties

- **Gebufferde I/O** - Gebruikt `BufWriter` voor optimale schrijf prestaties
- **Minimale allocaties** - Efficiënte string formattering en geheugen gebruik
- **Thread synchronisatie** - Mutex-beschermde schrijfbewerkingen voorkomen data corruptie
- **Lazy initialisatie** - Logger componenten worden alleen gemaakt wanneer nodig

## 📚 Voorbeelden

Volledige voorbeelden zijn beschikbaar in de `examples/` directory:

```bash
# Basis logging voorbeeld
cargo run --example basic_usage

# JSON gestructureerde logging  
cargo run --example json_logging

# Hoog-volume logging met rotatie
cargo run --example rotation_demo

# NIEUW: Geformatteerde logging voorbeelden
cargo run --example formatted_logging
```

## 📋 Vereisten

- Rust 1.70 of later
- Afhankelijkheden: chrono (timestamps), tempfile (dev/testing alleen)

## 🤝 Bijdragen

Bijdragen zijn welkom! Zie [CONTRIBUTING.md](CONTRIBUTING.md) voor richtlijnen.

## 🛣️ Roadmap

### v2.0.0 (Gepland)
- Database integratie voor directe log opslag
- Tijd-gebaseerde rotatie (dagelijks, wekelijks, maandelijks)
- Async logging voor hoge-prestatie applicaties
- Verbeterde caller locatie tracking
- Volledige custom patroon parsing

### Doorlopend
- Bug fixes en prestatie verbeteringen
- Aanvullende output formaten
- Uitgebreide platform ondersteuning

## 📄 Licentie

Gelicenseerd onder de **MIT Licentie**.

Zie [CHANGELOG](CHANGELOG.md) voor gedetailleerde versie geschiedenis.

## 🔗 Links

- **Issues**: [GitHub Issues](https://github.com/Jurgen-Be/FreedomLogger/issues)
- **Discussies**: [GitHub Discussions](https://github.com/Jurgen-Be/FreedomLogger/discussions)
- **Documentatie**: [docs.rs/freedom_logger](https://docs.rs/freedom_logger)

---

**Gebouwd met Rust voor prestaties, veiligheid en betrouwbaarheid.** 🦀