# Progetto 2.1: Georuggine --- Sistema di geolocalizzazione per una flotta di veicoli

Il progetto consiste nella realizzazione di un'applicazione  
client/server sviluppata in Rust, finalizzata alla gestione della  
geolocalizzazione e della comunicazione con una flotta di veicoli. Il  
sistema deve permettere a più utenti di registrarsi, autenticarsi,  
inviare periodicamente la propria posizione al server e comunicare con  
esso tramite messaggi di testo.

## Gestione degli utenti e della posizione

Ogni utente accede al sistema attraverso una fase di registrazione  
nella quale vengono definiti un account e una password. Una volta  
registrato, l'utente può essere monitorato dal server attraverso  
coordinate geografiche, gestite in modo emulato.

La posizione di ciascun utente viene trasmessa al server ogni 30  
secondi, così da consentire il tracciamento continuo degli spostamenti.

Il sistema deve inoltre gestire lo stato di ogni utente. In particolare,  
un utente può risultare:
- sconnesso;
- fermo;
- in movimento.

La transizione dallo stato **fermo** allo stato **in movimento** avviene  
quando viene rilevato un cambiamento delle coordinate. Al contrario, il  
passaggio dallo stato **in movimento** allo stato **fermo** si verifica  
quando, per almeno tre minuti, la posizione dell'utente non cambia.

## Simulazione del movimento

Per simulare il movimento dei veicoli è possibile adottare diverse  
strategie:

- lettura di un file contenente una sequenza di coordinate e tempi;

- definizione di un punto di partenza e di un punto di arrivo,  specificando anche eventuali pause lungo il tragitto;

- inserimento manuale delle coordinate tramite interfaccia;

- utilizzo di un generatore pseudo-casuale di posizioni.


## Analisi del movimento

Il server deve essere in grado di analizzare il movimento compiuto da  
uno specifico utente.

Le informazioni richieste riguardano:

- il tragitto percorso;
- la velocità media;
- la durata complessiva del movimento;
- la durata delle pause.


Tali dati devono poter essere calcolati su intervalli temporali  
programmabili, come:

- il giorno corrente;
- la settimana corrente;
- il mese corrente.

## Comunicazione tra server e utenti

Un'altra funzionalità importante riguarda la comunicazione tra server e  
utenti.

Il server deve poter inviare messaggi di testo:

- in **broadcast**, cioè a tutti gli utenti connessi;
- in modo **diretto**, a un singolo utente.

Allo stesso tempo, ogni utente deve poter inviare messaggi testuali al  
server.

## Requisiti tecnici e prestazionali

Dal punto di vista tecnico, l'applicazione deve essere eseguibile su  
almeno due piattaforme diverse tra:

- Windows;

- Linux;

- macOS;

- Android;

- ChromeOS;

- iOS.


È inoltre richiesto di prestare attenzione alle prestazioni del sistema,  
in particolare al consumo di tempo CPU e alla dimensione  
dell'applicativo.

Il server deve generare un file di log che riporti, ogni due minuti, i  
dettagli relativi al tempo di CPU utilizzato.

Infine, nel report descrittivo del progetto deve essere indicata anche  
la dimensione del file eseguibile prodotto.

---

# Documento dei Requisiti

## StakeHolder

- **Professore (Committente)**: Chi definisce la specifica del progetto, stabilisce i vincoli (es. uso di Rust, portabilità) e valuta la qualità del lavoro svolto.

- **Team di Sviluppo (Developer)**: Gli studenti che analizzano il dominio, formalizzano i requisiti e sviluppano il software.

- **Utente Client** : L'attore che interagisce con il client mobile/desktop per inviare la posizione GPS (emulata o da file) e scambiare messaggi con il server.

- **Gestore della Flotta** : L'attore che monitora gli spostamenti generali della flotta, consulta le metriche analitiche (velocità media, tempi di sosta) e invia i messaggi diretti o in broadcast


## Diagramma di Contesto


![[image/CD.png]]


## Tabella delle Interfacce per il sistema **GeoRuggine**

| Attore (_Actor_)   | Interfaccia Fisica (_Physical interface_)                  | Interfaccia Logica (_Logical interface_) |
| :----------------- | :--------------------------------------------------------- | :--------------------------------------- |
| **Client**         | Schermo, tastiera e mouse del terminale/workstation client | **GUI**                                  |
| **ServerOperator** | Schermo, tastiera e mouse del terminale/workstation server | **GUI**                                  |


## Requisiti di Funzionalità

### Requisiti Funzionali

| ID Requisito | Requisito / Funzionalità               | Descrizione del Requisito                                                                                                                                                                                             |
| ------------ | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **F1**       | **Gestione Utenti ed Autenticazione**  |                                                                                                                                                                                                                       |
| **F1.1**     | Registrazione Utente                   | Il sistema deve consentire la registrazione di un nuovo utente tramite credenziali.(Nome,Cognome,Email,Password,Conferma Password)                                                                                    |
| **F1.2**     | Login Utente                           | Il sistema deve autenticare un utente registrato tramite credenziali.(email e password)                                                                                                                               |
| **F1.3**     | Logout Utente                          | Il sistema deve consentire la disconnessione dell'utente autenticato.                                                                                                                                                 |
| **F2**       | **Tracciamento e Monitoraggio Flotta** |                                                                                                                                                                                                                       |
| **F2.1**     | Inviare Posizione                      | Il client deve trasmettere periodicamente (ogni 30s) le proprie coordinate geografiche al server.                                                                                                                     |
| **F2.2**     | Analizzare il cambio di movimento      | Il sistema deve calcolare e aggiornare lo stato dell'utente ==(_sconnesso_, _fermo_, _in movimento_)== gestendo le transizioni automatiche alla variazione delle coordinate o in assenza di esse per almeno 3 minuti. |
| **F3**       | **Messaggistica Testuale**             |                                                                                                                                                                                                                       |
| **F3.1**     | Inviare/ricevere messaggi diretti      | Il sistema deve supportare lo scambio bidirezionale di messaggi testuali tra singolo client e server.                                                                                                                 |
| **F3.2**     | Inviare messaggi broadcast             | Il server deve consentire l'invio di messaggi in broadcast rivolti all'intera flotta attiva.                                                                                                                          |
| **F4**       | **Analisi Metriche e Reportistica**    |                                                                                                                                                                                                                       |
| **F4.1**     | Gestire le finestre temporali          | Il sistema deve consentire la selezione e l'impostazione di intervalli temporali personalizzati (es. giorno, settimana, mese corrente).                                                                               |
| **F4.2**     | Elaborazione metriche                  | Il server deve calcolare per ciascun profilo il percorso complessivo, la velocità media, il tempo totale di spostamento e la durata delle pause.                                                                      |
| **F4.3**     | Generare file report                   | Il sistema deve generare report riassuntivi (ogni 2min) contenenti i risultati delle metriche analitiche elaborate.                                                                                                   |


### Requisiti non funzionali


| ID      | Categoria     | Descrizione del Requisito                                                                                                                             |
| ------- | ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| **NF1** | Sicurezza     | ==Le credenziali di autenticazione e lo storico delle posizioni geografiche devono essere memorizzati nel DB esclusivamente in formato cifrato.==     |
| **NF2** | Sicurezza     | Il canale di comunicazione socket tra client e server deve garantire la riservatezza dei dati in transito tramite cifratura.                          |
| **NF3** | Portabilità   | ==L'applicativo deve garantire la compilazione ed esecuzione su almeno due piattaforme a scelta tra Windows, Linux, macOS, Android, ChromeOS e iOS.== |
| **NF5** | Affidabilità  | Il server deve rimanere operativo e continuare il tracciamento della flotta anche in presenza di disconnessioni improvvise dei singoli client.        |
| **NF6** | Organizzativo | ==Il codice sorgente del  server deve essere sviluppato in linguaggio Rust e gestito tramite repository Git.==                                        |
| **NF7** | Usabilità     | La procedura di registrazione e primo accesso dall'interfaccia utente deve essere completata in non più di 5 passaggi di input.                       |
| NF8     | Efficienza    |                                                                                                                                                       |
### Requisiti di Dominio


| ID                        | Regola di Dominio |
| ------------------------- | ----------------- |
| Formula calcolo Posizione | Euclidea          |
| Formula Calcolo Metriche  | ?                 |

### Tabella dei diritti 

|                    | F1.1  | F1.2  | F1.3  | F2.1  | F2.2  | F3.1  | F3.2  | F4.1  | F4.2  | F4.3  |
| ------------------ | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- |
| **Client**         | **X** | **X** | **X** | **X** |       | **X** |       |       |       |       |
| **ServerOperator** |       |       |       |       | **X** | **X** | **X** | **X** | **X** | **X** |

## Casi D'uso e diagramma dei Casi d'uso



### Caso d'uso 1 (UC1) - Gestione account utente

- **Attori coinvolti:** Client, Operatore del server
- **Precondizione:** Nessuna
- **Post-condizione:** L'account utente è registrato o autenticato nel sistema
- **Scenario nominale:** L'utente registra un nuovo account specificando nome utente e password. L'utente accede al sistema con credenziali valide.
- **Varianti:**
	- Nome utente già esistente: notifica errore.
    - Credenziali non valide in fase di accesso: notifica errore.

#### Scenario 1-1: Registrazione nuovo account client

- **Precondizione:** L'utente `U` non esiste nel sistema
- **Post-condizione:** L'account `U` viene registrato nel sistema con stato iniziale _Sconnesso_

|**Passo**|**Descrizione**|
|---|---|
|**1**|Il client inserisce il nome utente `U`|
|**2**|Il client inserisce la password `P`|
|**3**|Il client invia i dati di registrazione|
|**4**|Il server verifica l'univocità di `U`|
|**5**|Il server memorizza le credenziali e conferma l'avvenuta registrazione|

#### Scenario 1-2: Autenticazione utente (Login)

- **Precondizione:** L'account `U` esiste nel sistema
- **Post-condizione:** L'utente `U` risulta autenticato e connesso


|**Passo**|**Descrizione**|
|---|---|
|**1**|Il client inserisce nome utente `U` e password `P`|
|**2**|Il server verifica la correttezza delle credenziali|
|**3**|Il server imposta lo stato dell'utente su _Fermo_|
|**4**|L'interfaccia client abilita le funzioni di simulazione e messaggistica|

### Caso d'uso 2 (UC2) - Trasmissione e tracciamento della posizione

- **Attori coinvolti:** Client, Operatore del server
- **Precondizione:** Il client ha effettuato l'accesso
- **Post-condizione:** Storico posizioni aggiornato e stato utente aggiornato (_Fermo_ / _In movimento_)
- **Scenario nominale:** Il client trasmette periodicamente le coordinate geografiche emulate ogni 30 secondi. Il server registra le coordinate e aggiorna lo stato dell'utente.

- **Varianti:**  
    - Coordinate invariate per almeno 3 minuti: il server imposta lo stato su _Fermo_.
    - Coordinate variate rispetto all'ultima rilevazione: il server imposta lo stato su _In movimento_.
    - Disconnessione o timeout del client: il server imposta lo stato su _Sconnesso_.


#### Scenario 2-1: Invio posizione periodica con passaggio a stato "In movimento"

- **Precondizione:** Il client `C` è connesso e il suo stato è _Fermo_
- **Post-condizione:** `C.ultimaPosizione = NuoveCoordinate`; `C.stato = In movimento`

| **Passo** | **Descrizione**                                                   |
| --------- | ----------------------------------------------------------------- |
| **1**     | Il timer periodico del client scade (30 secondi)                  |
| **2**     | Il client rileva la coordinata emulata corrente `NuoveCoordinate` |
| **3**     | Il client trasmette `NuoveCoordinate` al server                   |
| **4**     | Il server confronta `NuoveCoordinate` con la posizione precedente |
| **5**     | Il server rileva la variazione e imposta `C.stato = In movimento` |
| **6**     | Il server memorizza le coordinate con marca temporale             |

#### Scenario 2-2: Transizione allo stato "Fermo" per inattività

- **Precondizione:** Il client `C` è connesso e il suo stato è _In movimento_
- **Post-condizione:** `C.stato = Fermo`

|**Passo**|**Descrizione**|
|---|---|
|**1**|Il timer periodico del client scade (30 secondi)|
|**2**|Il client invia le coordinate correnti|
|**3**|Il server rileva l'assenza di variazioni rispetto all'ultima posizione|
|**4**|Il server verifica che il tempo senza variazioni sia >= 3 minuti|
|**5**|Il server aggiorna `C.stato` impostandolo su _Fermo_|

### Caso d'uso 3 (UC3) - Configurazione simulazione movimento

- **Attori coinvolti:** Client
- **Precondizione:** Il client ha effettuato l'accesso
- **Post-condizione:** Modulo di simulazione configurato e attivo
- **Scenario nominale:** L'utente seleziona la modalità con cui emulare il movimento: caricamento da file (coordinate e tempi), definizione di partenza/arrivo con soste, inserimento manuale o generazione pseudo-casuale.

- **Varianti:** 
    - File coordinate non trovato o corrotto: notifica errore.
    - Coordinate manuali non valide geograficamente: notifica errore.

#### Scenario 3-1: Caricamento simulazione da file di tracciato

- **Precondizione:** Il file di tracciato `F` esiste ed è formattato correttamente
- **Post-condizione:** Percorso client inizializzato a partire da `F`

|**Passo**|**Descrizione**|
|---|---|
|**1**|Il client seleziona la modalità di simulazione da file|
|**2**|Il client specifica il percorso del file `F`|
|**3**|Il client esegue il parsing di coordinate e tempi|
|**4**|Il client programma la sequenza dei punti da inviare|

#### Scenario 3-2: Definizione percorso partenza/arrivo con soste

- **Precondizione:** Il client ha effettuato l'accesso
- **Post-condizione:** Itinerario sintetico generato

|**Passo**|**Descrizione**|
|---|---|
|**1**|Il client inserisce punto di partenza `A` e punto di arrivo `B`|
|**2**|Il client definisce una o più pause intermedie con relativa durata|
|**3**|Il client conferma i parametri|
|**4**|Il client calcola i punti intermedi da trasmettere ogni 30 secondi|

### Caso d'uso 4 (UC4) - Analisi del movimento utente

- **Attori coinvolti:** Operatore del server
- **Precondizione:** L'utente selezionato esiste e dispone di dati di tracciamento
- **Post-condizione:** Metriche di movimento calcolate e visualizzate
- **Scenario nominale:** L'operatore seleziona un utente e sceglie un intervallo temporale (giorno, settimana o mese corrente). Il server calcola tragitto percorso, velocità media, durata complessiva del movimento e durata delle pause.
- **Varianti:**
    - Nessun dato registrato nell'intervallo temporale scelto: visualizzazione metriche a zero.
#### Scenario 4-1: Calcolo metriche di movimento per la settimana corrente

- **Precondizione:** L'operatore ha effettuato l'accesso; l'utente `U` ha registrazioni nella settimana corrente
- **Post-condizione:** Report settimanale mostrato a video

|**Passo**|**Descrizione**|
|---|---|
|**1**|L'operatore seleziona l'utente `U` dalla lista della flotta|
|**2**|L'operatore imposta il filtro temporale "Settimana corrente"|
|**3**|L'operatore invia la richiesta di calcolo|
|**4**|Il server estrae i punti registrati per `U` nell'intervallo temporale indicato|
|**5**|Il server calcola la distanza totale percorsa e la velocità media|
|**6**|Il server calcola la durata totale di movimento e la durata complessiva delle soste|
|**7**|Il sistema mostra il prospetto dei dati all'operatore|

## Caso d'uso 5 (UC5) - Gestione messaggistica di testo

- **Attori coinvolti:** Client, Operatore del server
- **Precondizione:** Almeno un utente è connesso al sistema
- **Post-condizione:** Messaggio recapitato e registrato nella sessione
- **Scenario nominale:** Il server invia messaggi testuali diretti o in broadcast ai client; il client invia messaggi testuali verso il server.

- **Varianti:**
    - Destinatario diretto disconnesso: notifica di mancata consegna.
    - Invio broadcast senza alcun client connesso: notifica informativa.

#### Scenario 5-1: Invio messaggio testuale 

- **Precondizione:** Il client `C` è connesso, Server `S` connesso
- **Post-condizione:** Messaggio `M` visualizzato sulla console dell'operatore server

|**Passo**|**Descrizione**|
|---|---|
|**1**|Il client `C` inserisce il testo del messaggio `M`|
|**2**|Il client `C` invia il messaggio|
|**3**|Il server riceve `M`|
|**4**|Il server mostra a video il messaggio `M` con mittente `C` e orario di ricezione|

#### Scenario 5-2: Invio messaggio in broadcast dal server alla flotta

- **Precondizione:** Almeno un client risulta connesso al server
- **Post-condizione:** Messaggio `M` recapitato a tutti i client collegati

|**Passo**|**Descrizione**|
|---|---|
|**1**|L'operatore inserisce il messaggio broadcast `M`|
|**2**|L'operatore conferma l'invio alla flotta|
|**3**|Il server individua tutti i client attualmente attivi|
|**4**|Il server inoltra il messaggio `M` a ciascuna sessione aperta|
|**5**|Ogni client riceve e mostra a video il messaggio `M`|

### Caso d'uso 6 (UC6) - Monitoraggio prestazioni server

- **Attori coinvolti:** Operatore del server
- **Precondizione:** Il server è in esecuzione
- **Post-condizione:** Dati sull'utilizzo della CPU salvati su file di log
- **Scenario nominale:** Il server rileva periodicamente (ogni due minuti) il tempo di CPU consumato dal processo e scrive la voce di log su disco.

#### Scenario 6-1: Scrittura periodica log CPU

- **Precondizione:** Processo server attivo; timer periodico scaduto (120 secondi)
- **Post-condizione:** Nuova voce aggiunta al file di log

| **Passo** | **Descrizione**                                                              |
| --------- | ---------------------------------------------------------------------------- |
| **1**     | Il timer interno del server raggiunge l'intervallo di 2 minuti               |
| **2**     | Il server interroga il sistema operativo per rilevare il tempo CPU consumato |
| **3**     | Il server formatta i valori registrati associandoli alla marca temporale     |
| **4**     | Il server scrive e salva la riga nel file di log dedicato                    |

![[image/useCases.png]]