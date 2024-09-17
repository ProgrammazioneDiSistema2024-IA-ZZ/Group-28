# Manuale del progettista

## Indice

- [Descrizione generale](#descrizione-generale)
- [Metodi e struttura interna della struct Mouse](#metodi-e-struttura-generale-della-struct-mouse)
- [Gestione comando esterno]
- [Scrittura sul file]
- [Inizio dell'operazione di back-up e finestre grafiche]
- [Riepilogo operazione]

## Descrizione generale

L'esecuzione dell'operazione è suddivisa in 4 fasi logiche, fisicamente separate fra loro dal campo n_fase della struct Mouse.  
Nella prima fase (n_fase=1 come da inizializzazione) si attende l'esecuzione del comando iniziale, il rettangolo che deve coprire l'intero schermo rimanendo all'interno di un range di accettabilità concepito come cornice interna all'interno della quale il comando è ancora valido. Una volta completato il comando iniziale il campo n_fase viene portato a 2 e in questa fase si attende un comando da tastiera che va a riempire i parametri "versione" e "formato" che verranno usati per il back-up. In questa fase buona parte dei tasti ('a'/'A' - 'm'/'M') è associata ad un comportamento specifico che va dall'operazione completa ('a'/'A'), che copia l'intero sottoalbero dalla directory sorgente a quella di destinazione, all'utilizzo del parametro "formato" che permette di copiare i soli file con quell'estensione. Una volta riempite queste variabili si passa alla terza fase (n_fase=3) in cui si attende il completamento del comando di conferma. Questo comando consiste in una linea orizzontale, da un lato all'altro dello schermo, a qualsiasi altezza, rimanendo anche in questo caso all'interno di un range di accettabilità. Le dimensioni dello schermo, in pixel, infine, sono gestite da due costanti il cui valore, però, è soggettivo, e quindi deve essere adattato alla risoluzione del PC in uso.

## Metodi e struttura generale della struct Mouse

Lo sviluppo dell'esecuzione e il progredire delle fasi sono gestiti interamente dalla struct Mouse, la quale, una volta inizializzata una variabile di tipo Mouse, rappresenterà il mouse. I campi interni vengono utilizzati non solo per gestire la posizione della freccia del mouse, ma anche per verificare la corretta esecuzione dei comandi di conferma mediante dei booleani. In particolare:  

- I campi pos_x e pos_y sono le coordinate della posizione del mouse rispettivamente lungo gli assi di larghezza e altezza dello schermo;  
- Il campo n_fase distingue le varie fasi logiche dell'esecuzione;  
- I campo pos_prec_y contiene la coordinata y alla quale è stato iniziato  il comando della conferma mediante la linea orizzontale. Dato il fatto che lo schermo potrebbe essere non agibile, l'altezza a cui eseguire il comando è lasciata libera, essendo impossibile per l'utente identificare la posizione del mouse senza l'uso del monitor;  
- I due booleani is_active e is_active_vec vengono utilizzati per la corretta esecuzione dei comandi dall'esterno: is_active_vec, array booleano di 4 posizioni, rappresenta i 4 angoli dello schermo, se sono stati toccati correttamente oppure no; l'altro campo invece monitora la corretta esecuzione del comando di conferma;  
- Gli ultimi due campi (range_rettangolo e range_conferma) sono pensati come range di accettabilità per i comandi con il mouse. Quello per il rettangolo è pensato come cornice interna essendo il comando da eseguire sui bordi dello schermo; l'altro, invece, è pensato per aiutare l'utente anche nel caso in cui la conferma sia eseguita con una linea a mezza altezza, essendo quest'ultimo comando più difficile da eseguire rispetto al precedente se non si ha il supporto dello schermo.
