# Manuale del progettista

## Indice

- [Descrizione generale](#descrizione-generale)
- [Avvio](#avvio)
- [Metodi e struttura interna della struct Mouse](#metodi-e-struttura-interna-della-struct-mouse)
- [Gestione risoluzione](#gestione-risoluzione)
- [Gestione comando esterno](#gestione-comando-esterno)
- [Segnali acustici](#segnali-acustici)
- [Scrittura sul file](#scrittura-sul-file)
- [Operazione di back-up](#operazione-di-back-up)
- [Finestre grafiche](#finestre-grafiche)
- [Riepilogo operazione](#riepilogo-operazione)

## Descrizione generale

L'esecuzione dell'operazione è suddivisa in 4 fasi logiche, fisicamente separate fra loro dal campo `n_fase` della struct Mouse.  
Nella prima fase (`n_fase=1` come da inizializzazione) si attende l'esecuzione del comando iniziale, il rettangolo che deve coprire l'intero schermo rimanendo all'interno di un range di accettabilità concepito come cornice interna nella quale il comando è ancora valido. Una volta completato il comando iniziale il campo `n_fase` viene portato a 2 e in questa fase si attende un comando da tastiera che va a riempire il parametro `formato` che verra usato per il back-up. In questa fase buona parte dei tasti ('a'/'A' - 'm'/'M') è associata ad un comportamento specifico che va dall'operazione completa ('a'/'A'), che copia l'intero sottoalbero dalla directory sorgente a quella di destinazione, alla copia dei soli file con quell'estensione specificata. Una volta riempita questa variabile si passa alla terza fase (`n_fase=3`) in cui si attende il completamento del comando di conferma. Questo comando consiste in una linea orizzontale, da un lato all'altro dello schermo, a qualsiasi altezza, rimanendo anche in questo caso all'interno di un range di accettabilità. L'ultima fase invece, l'unica a non essere caratterizzata da un valore di `n_fase`, ma che viene avviata dopo la conferma, è la vera e propria operazione di back-up, con le relative finestre grafiche, al termine della quale si ritorna alla fase iniziale. Le dimensioni dello schermo in pixel, infine, sono gestite da due costanti il cui valore, però, è soggettivo, e quindi deve essere adattato alla risoluzione del PC in uso.

## Avvio

Al momento dell'avvio del processo viene costruito subito l'oggetto eseguibile da salvare immediatamente come eseguibile automaticamente all'avvio. Per fare questo si usa il crate `auto-launch` e la sua libreria interna `AutoLaunchBuilder`.

- Come prima cosa si ottiene, attraverso la libreria `env`, il path dell'eseguibile, concatenando il metodo `current_exe()` con quelli necessari per ottenere un valore in formato `String`.
- A questo punto interviene `AutoLaunchBuilder` che, dopo aver settato il nome dell'applicazione, usa il path appena generato per costruire l'applicazione stessa. 
Nota: il path viene generato come `String` e successivamente trasformato  in `&str` per questioni legate ai tempi di vita.
- L'applicazione generate potrebbe già essere settata come automatica quindi bisogna verificarlo con il metodo `is_enabled()`:
  - nel caso non si riuscisse ad ottenere tale informazione si lancia un errore;
  - nel caso si riuscisse, se l'applicazione non è settata la si setta, altrimenti si avvia l'operazione di attesa comandi.  
Nota: Questo controllo è necessario in quanto i due file, quello che configura e quello da configurare, sono lo stesso file che, pertanto, riconfigurerebbe l'eseguibile ad ogni avvio.

## Metodi e struttura interna della struct Mouse

Lo sviluppo dell'esecuzione e il progredire delle fasi sono gestiti interamente dalla struct `Mouse`, la quale, una volta inizializzata una variabile di tipo `Mouse`, rappresenterà il mouse stesso. I campi interni vengono utilizzati non solo per gestire la posizione della freccia, ma anche per verificare la corretta esecuzione dei comandi di conferma mediante dei booleani. In particolare:

- I campi `pos_x` e `pos_y` sono le coordinate della posizione del mouse rispettivamente lungo gli assi di larghezza e altezza schermo;
- Il campo `n_fase` distingue le varie fasi logiche dell'esecuzione;
- I campo `pos_prec_y` contiene la coordinata y alla quale è stato iniziato il comando della conferma mediante la linea orizzontale. Dato il fatto che lo schermo potrebbe essere non agibile, l'altezza a cui eseguire il comando è lasciata libera, essendo impossibile per l'utente identificare la posizione del mouse senza l'uso del monitor;
- I due booleani `is_active` e `is_active_vec` vengono utilizzati per la corretta esecuzione dei comandi dall'esterno: `is_active_vec`, array booleano di 4 elementi, rappresenta i 4 angoli dello schermo, se sono stati toccati correttamente oppure no; l'altro campo invece monitora la corretta esecuzione del comando di conferma;
- Gli ultimi due campi (`range_rettangolo` e `range_conferma`) sono pensati come range di accettabilità per i comandi con il mouse. Quello per il rettangolo è pensato come cornice interna essendo il comando da eseguire sui bordi dello schermo; l'altro, invece, è pensato per aiutare l'utente anche nel caso in cui la conferma sia eseguita con una linea a mezza altezza, essendo quest'ultimo comando più difficile da eseguire rispetto al precedente se non si ha il supporto dello schermo.

La struct `Mouse` implementa anche dei metodi utilizzati per gestire i vari eventi (click sul mouse o pressione di un tasto):

- `new` è il costruttore, che setta `n_fase` a 1, le coordinate a 0, i range ai valori prestabiliti e i booleani a `false`;
- `inizio_attesa` viene utilizzato quando, in fase 1, si clicca con il mouse. Se la posizione della freccia è nell'intorno dell'angolo in alto a sinistra, considerando il range, il suddetto angolo viene etichettato come "cliccato" nel vettore `is_active_vec`.
- `fine_attesa`, invece, viene utilizzato sempre per il rettangolo, ma nel caso in cui viene rilasciato il mouse. Se di nuovo la posizione è nell'intorno dell'angolo in alto a sinistra e questa volta il vettore dei booleani contiene 4 `true`, allora si passa in fase 2.
- `cambia_posizione_per_rettangolo` è il metodo che gestisce il movimento del mouse nel caso in cui `n_fase` sia 1. Una volta ricevute, dalla funzione `main`, le nuove coordinate, si verifica che la freccia del mouse sia ancora all'interno della cornice valida per il comando: a seconda del vettore `is_active_vec`, si risale al prossimo angolo che dovrebbe essere visitato durante il comando. Una volta trovato si verifica che la nuova posizione non ecceda i limiti previsti dal range: se così è l'intero vettore viene azzerato e l'operazione deve essere eseguita da capo; sennò, nel caso il prossimo angolo sia stato raggiunto, lo si segnala attraverso `is_active_vec`. Infine si sovrascrivono i campi `pos_x` e `pos_y` con i nuovi valori ricevuti;
- `attivazione_conferma` è il metodo duale a `inizio_attesa`, però per il comando di conferma. Se il mouse, qualunque sia il valore di `pos_y`, si trova all'estremo sinistro, o lì intorno, viene salvato il valore di `pos_y` nel campo `pos_prec_y` in modo da avere un riscontro alla fine del comando per vedere se è stato tracciato correttamente. Inoltre, viene portato a `true` il booleano, in modo da distinguere una corretta esecuzione da una che, nonostante abbia portato la freccia a destra, non ha rispettato il range, o sia stata interrotta rilasciando il mouse in anticipo;
- `disattivazione_conferma`, invece, si occupa del rilascio del mouse in fase di conferma. Anche qui: se ci troviamo all'estremo destro, considerando il range, e il booleano segna che l'operazione è ancora vaida si lancia direttamente la funzione di back-up, passando come parametro la variabile ricevuta dal metodo (formato);
- `cambia_posizione_per_conferma`, infine, gestisce il movimento nella fase 3. Come in fase 1, una volta ricevute le nuove coordinate, si verifica che la coordinata y non sfori il range prestabilito (range separato pensato per la sola conferma). Questo controllo viene fatto qui e non ripetuto in `disattivazione_conferma` in quanto ridondante.

## Gestione risoluzione

Per gestire correttamente le coordinate e lo spostamento del mouse è necessario conoscere la risoluzione dello schermo in pixel; questo viene fatto dalla funzione `ottieni_dimensioni`, che le ottiene attraverso un comando sul terminale. Per farlo prima di tutto bisogna generare il processo:

- Attraverso la funzione `Command::new("cmd")` si istanzia un nuovo processo che agisce sul terminale;
- Con il metodo `args` si passano al processo figlio gli argomenti, ossia il codice per ottenere la risoluzione;
- Con il metodo `output()` si attende la terminazione prelevando il risultato;  
Nota: il processo potrebbe non essere stato creato a causa di un qualsiasi errore, pertanto si gestisce questa eventualità con `expect()`.

Ora nella variabile `program`, oltre ad altre informazioni, è presente la risoluzione del display. Per ottenerla:

- Si estrae il risultato attraverso `String::from_utf8_lossy(&programm.stdout)`;
- Con il metodo `lines()` si trasforma il risultato in una collezione di stringhe;
- Si preleva solo la linea effettivamente utile con `nth(1).unwrap().to_string()`;
- Si ottengono i valori separando la stringa in corrispondenza del separatore (25 spazi) e con `nth(n).unwrap()` si prendono in 2 variabili;
- Si trasformano i valori ottenuti in valori interi con `parse::<i32>().expect("Errore di conversione)`;  
Nota: il secondo valore ottenuto dopo la separazione contiene spazi dopo che devono essere eliminati, prima della conversione, con il metodo `trim()`.

## Gestione comando esterno

La gestione del comando dall'esterno rappresenta il passo più cruciale e delicato dell'intero programma. Una volta inizializzata la variabile che rappresenta il mouse, il thread principale, attraverso la funzione `listen`, viene messo in attesa di un evento dall'esterno. Questa attesa non è sbloccabile, a meno che non si interrompa lo stesso thread, cosa che permette l'esistenza di più fasi separate senza creare problemi una volta deciso come distinguerle (campo `n_fase`). La funzione `listen` accetta come parametro una calback (`callback_parametro`) che, una volta ricevuto in ingresso l'evento stesso, passato automaticamente da `listen`, accede al campo `event_type` il quale, attraverso l'enumerazione `EventType`, contiene il tipo dell'evento (`ButtonPress`, `ButtonRelease`, `KeyPress`...). Ovviamente il comportamento in risposta all'evento deve essere diverso a seconda della fase, il che porta ad un controllo a monte del valore di `n_fase`.

- `n_fase=1`: si usano i metodi `inizio_attesa`, `cambia_posizione_per_rettangolo` e `fine_attesa` per gestire gli eventi rispettivamente di tipo `ButtonPress`, `MouseMove` e `ButtonRelease` (quest'ultimo gestirà eventualmente il passaggio alla fase 2);
- `n_fase=2`: il singolo evento accettato in questa fase è il `ButtonPress` e il valore di `n_fase` viene immediatamente portato a 3 qualunque sia il tasto premuto, ma se poi non è uno dei tasti accettati si ritorna in fase 1. Una volta eseguito l'unwrap del codice del tasto si controlla, attraverso un match, se rispecchia uno dei casi accettati e, se così è, si riempie il parametro `formato` secondo la logica;
- `n_fase=3`: in quest'ultimo caso vengono invocati i metodi `cambia_posizione_per_conferma`, `attivazione_conferma` e `disattivazione_conferma` per gestire gli stessi eventi citati sopra, più un evento, la pressione di un tasto, che permette di resettare l'operazione. Nell caso in cui `disattivazione_conferma` riconosca l'effettivo completamento del comando chiama la funzione di back up passandole come parametro `formato`, il campo di cui ha bisogno per operare correttamente.

## Segnali acustici

Durante la preparazione al backup e l'interazione con l'utente vengono attivate delle segnalazioni acustiche costruite attraverso il crate `rodio` e le sue funzioni interne `source::SineWave` per generare il suono, OutputStrem per generare il canale audio e Sink per riprodurlo.
Ogni volta che si vuole riprodurre l'audio viene chiamata la funzione `suono_comando` che riceve in ingresso il numero di volte che deve essere riprodotto il suono. I passaggi da seguire sono:

- la chiamata alla funzione `suono_comando` passando in ingresso un intero;
- l'apertura della coppia (stream,stream_handle) attraverso `OutputStream::try_default().unwrap`;
- la generazione della coda segnali da riprodurre con `Sink::try_new(&stream_handle).unwrap()` passando lo stream creato al punto 2;
- in un iterazione basata sul numero passato come parametro:

  - si genera il segnale audio attraverso `SineWave::new(frequenza)`;
  - lo si aggiunge in coda consumandolo (da cui la necessità di rigenerarlo nell'iterazione successiva);
  - lo si riproduce con `sleep_until_end()`;
  - si attende per uun tempo preimpostato così da permettere la separazione fra i segnali;

I segnali vengoono riprodotti:

- al completamento del segnale iniziale (doppio) o al fallimento di quest'ultimo (singolo segnale);
- se viene premuto un pulsante non valido (singolo segnale);
- al completamento del segnale di conferma (doppio) o al fallimento dell'operazione (singolo segnale);
- al completamento dell'operazione di back-up (ttriplo segnale);

## Scrittura sul file

Come richiesta aggiuntiva al back-up si vuole che, ogni 2 minuti, venga scritto su un file il consumo di CPU da parte del processo. Questa operazione è svolta dal thread secondario che esegue la funzione `scrivi_ogni_tanto`. Il corpo della funzione è quasi completamente composto da un loop senza fine che, dopo aver atteso mediante la funzione sleep per 2 minuti:

- rinfresca le informazioni del sistema (istruzione necessaria per avere dati affidabili sul sistema);
- legge l'ora attraverso la libreria `Local` del crate esterno `chrono`;
- ricava, attraverso il metodo `process` e il pid del processo corrente ottenuto con il metodo `get-current-pid`, la percentuale di utilizzo della CPU per mezzo del metodo `cpu_usage`;
- costruisce il messaggio da scrivere sfruttando i valori ricavati (ora, pid e percentuale);
- scrive sul file (se non riesce a scrivere, per qualsiasi motivo, interrompe il loop).

## Operazione di back-up

Una volta completato il comando di conferma, nel metodo `disattivazione_conferma`, viene lanciata la funzione che esegue l'operazione di back-up, prendendo in ingresso il parametro rappresentante l'eventuale formato dei file che bisogna copiare.

All'interno della funzione a prima cosa da fare è leggere dal file `Info.txt` le directory di sorgente e destinazione: questo viene fatto nella funzione `leggi_info` che, dopo aver letto il contenuto del file usando il metodo `lines` che genera un iteratore di stringhe che rappresentano le linee del file, restituisce una tupla contenente le prime 2 stringhe dell'iteratore. Il metodo `lines`, da implementazione, restituisce un iteratore di tipo `Result`, ma per ottenere le prime 2 stringhe senza utilizzare un'iterazione viene usato il metodo `next` degli iteratori che restituisce un dato di tipo `Option` che contiene il tipo precedente, a giustifica dei 2 `unwrap` nel valore di ritorno.

Una volta ottenute le directory, dopo la dichiarazione di tutte le variabili utili all'operazione, come per esempio la dimensione totale dei file `dim_totale`, o il numero di file spostati `n_file`, e dopo aver verificato l'esistenza della directory sorgente e aver creato la directory di destinazione viene generata la finestra di conferma a cui vengono passate le directory, ottenute per mezzo della funzione `leggi_info`, per essere stampate nel messaggio di conferma. Una volta generata la finestra si prosegue richiamando la funzione `now` della libreria `ProcessTime` per cronometrare l'esecuzione del back-up in termini di cicli di CPU impiegati e si analizza il contenuto del parametro `formato`: se è `None` si esegue la versione completa del back-up, copiando l'intera directory sorgente senza distinzioni sul formato; altrimenti si esegue la versione ridotta passando come parametro la stringa contenuta dentro `formato`.

Le 2 versioni dell'operazione sono sostanzialmente identiche e differiscono solo in alcuni punti, ma in ognuno dei casi:

- Si tenta di aprire la directory passata come sorgente attraverso il metodo `read_dir` che restituisce un oggetto di tipo `Option`; nel caso questo contenga un oggetto lo si preleva ed assegna ad una variabile, sennò si ritorna al chiamante una tupla di valori nulli, per permettere al back-up di proseguire tranquillamente con eventuali altre cartelle;
- L'oggetto preso mediante `read_dir` è iterabile, di conseguenza mediante un ciclo `for` si può avere accesso ad ogni file e/o directory contenuta in quella analizzata;
- Una volta ottenuto il nome del nodo come stringa attraverso il metodo `file_name` si ottiene il path sorgente concatenando la directory parametro con il valore appena ottenuto, in quanto sono trattate come 2 semplici stringhe, separandoli mediante "/". La stessa cosa viene fatta utilizzando la stringa della directory di destinazione, al fine di costruire il nuovo path per il nodo;
- A questo punto bisogna separare i 2 casi, ossia il caso in cui il nodo sia un file o una directory, basandosi sui metodi `file_type`, `is_file` e `is_dir`.

  - Nel primo caso si confronta l'eventuale formato richiesto con quello ottenuto mediante il metodo `extension` e, nel caso coincidano, si usa la funzione `copy` per copiare il file da un path all'altro (nel caso in cui non sia specificato un formato richiesto si copia direttamente il file). Se l'operazione non da errori si considera il file come correttamente copiato e si incrementano le variabili rappresentanti il numero di file copiati e la dimensione totale, rispettivamente `n_file` e `dim`, di 1 nel caso di `n_file` e del risultato di un'operazione più complessa (metodi `metadata` e `len`) sul file nel caso di `dim`.

  - Nel secondo caso, invece, bisogna prima creare la directory (nel caso si copi l'intero sottoalbero) e poi copiare i file: prima si invoca la funzione `create_dir` che restituisce un `Result` che indica l'esito dell'operazione e poi si richiama la funzione di copiatura passando come parametri non i valori ricevuti, bensì le directory costruite con il nome del nodo. Al termine della ricorsione i valori ritornati (dimensione, numero file e eventuale numero directory copiate) vengono sommati a quelli contenuti nelle variabili della funzione.
- Alla fine del ciclo `for` si ritornano le 3 variabili citate (nel caso in cui si copi un formato specifico non si ritorna il numero delle cartelle poichè non considrate nel back-up). Dopodichè viene creato il file di riepilogo nella stessa directory di destinazione, utilizzando le informazioni appena ritornate dalla funzione di back-up.

## Finestre grafiche

La funzione di back-up genera anche una finestra grafica per segnalare ò'effettivo inizio dell'operazione, oppure un errore di creazione o lettura di una directory:

- Nel primo caso, nella funzione `crea_finestra_successo`, una volta passate le 2 directory tra le quali operare, si utilizza la libreria `Druid` per creare una finestra contenente un messaggio, creato nella funzione stessa, che contenga le due directory e l'ora letta attraverso `Local`;
- Nel secondo caso, invece, si passa alla funzione `crea_finestra_errore` una stringa di errore che verrà stampata nella finestra costruita nello stesso modo.

I problema delle finestre create e lanciate con `Druid` sta nel fatto che l'esecuzione del thread che le genera resta bloccata finchè queste non vengono chiuse, ma quando questo capita, essendo aperta solo una finestra in ogni caso, il thread stesso viene bloccato mandando in `panic` il processo essendo il thread principale. Per risolvere questo problema le finestre devono essere lanciate da thread secondari il cui blocco non rappresenta un problema o un rallentamento per il main-thread.

## Riepilogo operazione

Al termine dell'operazione di back-up, qualunque essa sia, viene generato un messaggio inteso come descrizione del processo appena concluso: questo, assieme alle variabili ritornate (`dim_totale`,`n_file` e l'eventuale `n_cartelle`), alla stringa rappresentante la directory di destinazione e alla variabile cronometro, interrotto e trascritto in micro-secondi, vengono passati alla funzione `crea_riepilogo`. Questa funzione serve per creare un riepilogo finale, nella directory di destinazione, in cui sono riassunti tutti i dettagli dell'operazione. In questa funzione:

- viene letta l'ora attraverso `Local`;
- viene definito il messaggio da scrivere includendo tutti i valori passati;
- la directory di destinazione viene utilizzata per creare il path sotto il quale viene creato il file di riepilogo;
- si tenta di scrivere il messaggio sul file (se non si ha successo si manda in `panic` il processo).
