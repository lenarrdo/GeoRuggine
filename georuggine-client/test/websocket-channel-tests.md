# Test manuali: invio messaggi WebSocket

Questi test servono a verificare il percorso di accodamento e trasmissione già presente nel client. Non modificano il codice dell'applicazione.

## Prerequisiti

- Avviare il server WebSocket previsto dal progetto.
- Avviare il client Tauri e autenticarsi, così da stabilire la connessione.
- Aprire la console/log del server per verificare i messaggi ricevuti.

## Casi di test

### 1. Invio di un messaggio di chat

1. Connettersi al server dal client.
2. Inviare un messaggio dalla chat.
3. Verificare che il server riceva un messaggio di tipo testo con contenuto e timestamp.

**Esito atteso:** il messaggio viene accodato tramite il sender condiviso e poi trasmesso dal task WebSocket.

### 2. Invio di coordinate

1. Connettersi al server.
2. Avviare il tracking caricando un file di coordinate valido.
3. Attendere l'intervallo configurato (30 secondi) per ciascun invio.
4. Verificare nei log del server la ricezione degli aggiornamenti di posizione.

**Esito atteso:** il tracking accoda gli aggiornamenti e il task di invio li trasmette in sequenza.

### 3. Invii concorrenti di chat e coordinate

1. Avviare il tracking.
2. Inviare un messaggio di chat mentre il tracking è attivo.
3. Verificare che arrivino entrambi i tipi di messaggio.

**Esito atteso:** i producer condividono il sender; il singolo consumer riceve i messaggi accodati e li invia uno alla volta. L'ordine tra producer concorrenti dipende da quale messaggio viene accodato per primo.

### 4. Invio senza connessione

1. Disconnettersi o non effettuare il login.
2. Tentare di inviare un messaggio di chat o avviare il tracking.

**Esito atteso:** l'invio viene rifiutato perché il WebSocket non è connesso; il chiamante riceve un errore.

### 5. Disconnessione durante il tracking

1. Avviare il tracking con più coordinate.
2. Disconnettersi mentre il tracking è attivo.
3. Verificare che i task di tracking, invio e ricezione vengano interrotti e che i successivi invii non vadano a buon fine.

**Esito atteso:** la disconnessione rimuove il sender condiviso e arresta i task associati alla connessione.

### 6. Carico di messaggi

1. Con connessione attiva, inviare molti messaggi di chat in rapida successione.
2. Osservare i messaggi ricevuti dal server e l'uso di memoria del client durante il test.

**Esito atteso:** i messaggi vengono accodati e trasmessi in sequenza. Poiché il canale è unbounded, se la produzione supera la velocità di trasmissione la coda può crescere; questo test non implica che il sistema applichi backpressure.

## Nota

Questi sono test manuali/documentativi, non test automatici eseguiti dal progetto. Per test ripetibili e automatici servirebbe un server WebSocket di test o un mock del trasporto, oltre a verifiche delle strutture e dei messaggi serializzati.
