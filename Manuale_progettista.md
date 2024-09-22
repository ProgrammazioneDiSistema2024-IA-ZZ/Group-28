# Manuale del progettista

## Indice

- [Descrizione generale](#descrizione-generale)
- [Metodi e struttura interna della struct Mouse](#metodi-e-struttura-interna-della-struct-mouse)
- [Gestione comando esterno](#gestione-comando-esterno)
- [Scrittura sul file](#scrittura-sul-file)
- [Operazione di back-up]
- [Finestre grafiche]
- [Riepilogo operazione]

## Descrizione generale

L'esecuzione dell'operazione è suddivisa in 4 fasi logiche, fisicamente separate fra loro dal campo n_fase della struct Mouse.  
Nella prima fase (n_fase=1 come da inizializzazione) si attende l'esecuzione del comando iniziale, il rettangolo che deve coprire l'intero schermo rimanendo all'interno di un range di accettabilità concepito come cornice interna nella quale il comando è ancora valido. Una volta completato il comando iniziale il campo n_fase viene portato a 2 e in questa fase si attende un comando da tastiera che va a riempire i parametri "versione" e "formato" che verranno usati per il back-up. In questa fase buona parte dei tasti ('a'/'A' - 'm'/'M') è associata ad un comportamento specifico che va dall'operazione completa ('a'/'A'), che copia l'intero sottoalbero dalla directory sorgente a quella di destinazione, all'utilizzo del parametro "formato" che permette di copiare i soli file con quell'estensione. Una volta riempite queste variabili si passa alla terza fase (n_fase=3) in cui si attende il completamento del comando di conferma. Questo comando consiste in una linea orizzontale, da un lato all'altro dello schermo, a qualsiasi altezza, rimanendo anche in questo caso all'interno di un range di accettabilità. L'ultima fase invece, l'unica a non essere caratterizzata da un valore di n_fase, ma che viene avviata dopo la conferma, è la vera e propria operazione di back-up, con le relative finestre grafiche, al termine della quale n_fase viene riportato ad 1. Le dimensioni dello schermo in pixel, infine, sono gestite da due costanti il cui valore, però, è soggettivo, e quindi deve essere adattato alla risoluzione del PC in uso.

## Metodi e struttura interna della struct Mouse

Lo sviluppo dell'esecuzione e il progredire delle fasi sono gestiti interamente dalla struct Mouse, la quale, una volta inizializzata una variabile di tipo Mouse, rappresenterà il mouse stesso. I campi interni vengono utilizzati non solo per gestire la posizione della freccia, ma anche per verificare la corretta esecuzione dei comandi di conferma mediante dei booleani. In particolare:

- I campi pos_x e pos_y sono le coordinate della posizione del mouse rispettivamente lungo gli assi di larghezza e altezza schermo;
- Il campo n_fase distingue le varie fasi logiche dell'esecuzione;
- I campo pos_prec_y contiene la coordinata y alla quale è stato iniziato il comando della conferma mediante la linea orizzontale. Dato il fatto che lo schermo potrebbe essere non agibile, l'altezza a cui eseguire il comando è lasciata libera, essendo impossibile per l'utente identificare la posizione del mouse senza l'uso del monitor;
- I due booleani is_active e is_active_vec vengono utilizzati per la corretta esecuzione dei comandi dall'esterno: is_active_vec, array booleano di 4 elementi, rappresenta i 4 angoli dello schermo, se sono stati toccati correttamente oppure no; l'altro campo invece monitora la corretta esecuzione del comando di conferma;
- Gli ultimi due campi (range_rettangolo e range_conferma) sono pensati come range di accettabilità per i comandi con il mouse. Quello per il rettangolo è pensato come cornice interna essendo il comando da eseguire sui bordi dello schermo; l'altro, invece, è pensato per aiutare l'utente anche nel caso in cui la conferma sia eseguita con una linea a mezza altezza, essendo quest'ultimo comando più difficile da eseguire rispetto al precedente se non si ha il supporto dello schermo.

La struct Mouse implementa anche dei metodi utilizzati per gestire i vari eventi (click sul mouse o pressione di un tasto):

- new è il costruttore, che setta n_fase a 1, le coordinate a 0, i range ai valori prestabiliti e i booleani a "false";
- inizio_attesa viene utilizzato quando, in fase 1, si clicca con il mouse. Se la posizione della freccia è nell'intorno dell'angolo in alto a sinistra, considerando il range, il suddetto angolo viene etichettato come "cliccato" nel vettore is_active_vec.
- fine_attesa, invece, viene utilizzato sempre per il rettangolo, ma nel caso in cui viene rilasciato il mouse. Se di nuovo la posizione è nell'intorno dell'angolo in alto a sinistra e questa volta il vettore dei booleani contiene 4 "true", allora si passa in fase 2.
- cambia_posizione_per_rettangolo è il metodo che gestisce il movimento del mouse nel caso in cui n_fase sia 1. Una volta ricevute, dalla funzione main, le nuove coordinate, si verifica che la freccia del mouse sia ancora all'interno della cornice valida per il comando: a seconda del vettore is_active_vec, si risale al prossimo angolo che dovrebbe essere visitato durante il comando. Una volta trovato si verifica che la nuova posizione non ecceda i limiti previsti dal range: se così è l'intero vettore viene azzerato e l'operazione deve essere eseguita da capo; sennò, nel caso il prossimo angolo sia stato raggiunto, lo si segnala attraverso is_active_vec. Infine si sovrascrivono i campi pos_x e pos_y con i nuovi valori ricevuti;
- attivazione_conferma è il metodo duale a inizio_attesa, però per il comando di conferma. Se il mouse, qualunque sia a coordinata di pos_y, si trova all'estremo sinistro, o lì intorno, viene salvato il valore di pos_y nel campo pos_prec_y in modo da avere un riscontro alla fine del comando per vedere se è stato tracciato correttamente. Inoltre viene portato a "true" il booleano, in modo da distinguere una corretta esecuzione da una che, nonostante abbia portato la freccia a destra, non ha rispettato il range, o il pulsante del mouse è stato rilasciato prima del previsto;
- disattivazione_conferma, invece, si occupa del rilascio del mouse in fase di conferma. Anche qui: se ci troviamo all'estremo destro, considerando il range, e il booleano segna che l'operazione è ancora vaida si lancia direttamente la funzione di back-up, passando come parametri le due variabili ricevute dal metodo (versione e formato);
- cambia_posizione_per_conferma, infine, gestisce il movimento nella fase 3. Come in fase 1, una volta ricevute le nuove coordinate, si verifica che la coordinata y non sfori il range prestabilito (range separato pensato per la sola conferma). Questo controllo viene fatto qui e non ripetuto in disattivazione_conferma in quanto ridondante.

## Gestione comando esterno

La gestione del comando dall'esterno rappresenta il passo più cruciale e delicato dell'intero programma. Una volta inizializzata la variabile che rappresenta il mouse, il thread principale, attraverso la funzione "listen", viene messo in attesa di un evento dall'esterno. Questa attesa non è sbloccabile, a meno che non si interrompa lo stesso thread, cosa che permette l'esistenza di più fasi separate senza creare problemi una volta deciso come distinguere le fasi (campo n_fase). La funzione "listen" accetta come parametro una calback (callback_parametro) che, una volta ricevuto in ingresso l'evento stesso, passato automaticamente da "listen", accede al campo "event_type" il quale, attraverso l'enumerazione EventType, contiene il tipo dell'evento (ButtonPress, ButtonRelease, KeyPress...). Ovviamente il comportamento in risposta all'evento deve essere diverso a seconda della fase, il che porta ad un controllo a monte del valore di n_fase.

- n_fase=1: si usano i metodi "inizio_attesa", "cambia_posizione_per_rettangolo" e "fine_attesa" per gestire gli eventi rispettivamente di tipo "ButtonPress", "MouseMove" e "ButtonRelease" (quest'ultimo eventualmente gestirà il passaggio alla fase 2);
- n_fase=2: il singolo evento accettato in questa fase è il "ButtonPress" e il valore di n_fase viene immediatamente portato a 3 qualunque sia il tasto premuto, ma se poi non è uno dei tasti accettati si ritorna in fase 1. Una volta eseguito l'unwrap del codice del tasto si controlla, attraverso un match, se rispecchia uno dei casi accettati e, se così è, si riempiono i parametri "versione" e "formato" secondo la logica;
- n_fase=3: in quest'ultimo caso vengono invocati i metodi "cambia_posizione_per_conferma", "attivazione_conferma" e "disattivazione_conferma" per gestire gli stessi eventi citati sopra, più un evento, la pressione di un tasto, che permette di resettare l'operazione. Nell caso in cui "disattivazione_conferma" riconosca l'effettivo completamento del comando chiama la funzione di back up passandole come parametri "versione" e "formato", i 2 campi di cui ha bisogno per operare correttamente.

## Scrittura sul file
Come richiesta aggiuntiva al back-up si vuole che, ogni 2 minuti, venga scritto su un file il consumo di CPU da parte del processo. Questa
operazione è svolta dal thread secondario che esegue la funzione scrivi_ogni_tanto. Il corpo della funzione è quasi completamente composto da un
loop senza fine che, dopo aver atteso mediante la funzione sleep per 2 minuti:

- rinfresca le informazioni del sistema (istruzione necessaria per avere dati affidabili sul sistema);
- legge l'ora attraverso la libreria "Local" del crate esterno "chrono";
- ricava, attraverso il metodo "process" e il pid del processo corrente, la percentuale di utilizzo della CPU per mezzo del metodo "cpu_usage";
- costruisce il messaggio da scrivere sfruttando i valori ricavati (ora,pid e percentuale);
- scrive (se non riesce a scrivere, per qualsiasi motivo, interrompe il loop).