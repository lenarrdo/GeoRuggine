# Georuggine

## 1 - Requisiti Implementativi

Il progetto consiste nella realizzazione di un'applicazione client/server sviluppata in Rust, finalizzata alla gestione della geolocalizzazione e della comunicazione con una flotta di veicoli. Il sistema deve permettere a più utenti di registrarsi, autenticarsi, inviare periodicamente la propria posizione al server e comunicare con esso tramite messaggi di testo.

### 1.1 - Registrazione e Autenticazione

Ogni utente accede al sistema attraverso una fase di registrazione, nella quale vengono definiti un account e una password. Una volta registrato, l'utente può essere monitorato dal server attraverso coordinate geografiche, gestite in modo emulato. La posizione di ciascun utente viene trasmessa al server ogni 30 secondi, così da consentire il tracciamento continuo degli spostamenti.

### 1.2 - Gestione del Movimento

Il sistema deve inoltre gestire lo stato di ogni utente. In particolare, un utente può risultare sconnesso, fermo oppure in movimento. La transizione dallo stato `fermo` allo stato `in movimento` avviene quando viene rilevato un cambiamento delle coordinate. Al contrario, il passaggio dallo stato `in movimento` allo stato `fermo` si verifica quando, per almeno tre minuti, la posizione dell'utente non cambia.

Per simulare il movimento dei veicoli è possibile adottare diverse strategie. Una soluzione può consistere nella lettura di un file contenente una sequenza di coordinate e tempi. In alternativa, si può definire un punto di partenza e un punto di arrivo, specificando anche eventuali pause lungo il tragitto. Altre possibilità includono l'inserimento manuale delle coordinate tramite interfaccia oppure l'utilizzo di un generatore pseudo-casuale di posizioni.

Il server deve essere in grado di analizzare il movimento compiuto da uno specifico utente. Le informazioni richieste riguardano il tragitto percorso, la velocità media, la durata complessiva del movimento e la durata delle pause. Tali dati devono poter essere calcolati su intervalli temporali programmabili, come il giorno corrente, la settimana corrente o il mese corrente.

### 1.3 - Comunicazione Server-Utenti

Un'altra funzionalità importante riguarda la comunicazione tra server e utenti. Il server deve poter inviare messaggi di testo sia in broadcast, cioè a tutti gli utenti connessi, sia in modo diretto a un singolo utente. Allo stesso tempo, ogni utente deve poter inviare messaggi testuali al server.

## 2 - Requisiti Tecnici

- [ ] L'applicazione deve essere eseguibile su almeno due piattaforme diverse tra Windows, Linux, MacOS, Android, ChromeOS e iOS.
- [ ] Ridotta dimensione dell'applicativo (includere dimensione dell'eseguibile nel report finale).
- [ ] Basso utilizzo di CPU time (In un file di log, il server riporta, _ogni due minuti_, i dettagli relativi al tempo di CPU utilizzato).

In sintesi, il progetto richiede lo sviluppo di un sistema distribuito in Rust capace di gestire utenti, posizioni geografiche simulate, stati di movimento, analisi dei percorsi e comunicazione client/server. L'obiettivo è realizzare un'applicazione efficiente, multipiattaforma e ben strutturata, prestando attenzione sia agli aspetti funzionali sia a quelli prestazionali.
