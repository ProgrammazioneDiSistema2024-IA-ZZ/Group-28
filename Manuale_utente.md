# Manuale utente

## Indice

- [Descrizione e scopo](#descrizione-e-scopo)
- [Preparazione](#preparazione)
- [Comando iniziale](#comando-iniziale)
- [Definizione eventuale formato](#definizione-eventuale-formato)
- [Comando di conferma](#comando-di-conferma)
- [Conclusione operazione](#conclusione-operazione)

## Descrizione e scopo

Quest'applicazione ha il compito di aiutare ad effettuare il back-up totale/parziale dei file da una dirctory ad un'altra, ed è specificatamente pensata per operare qualora lo schermo non sia agibile, attraverso comandi dal mouse e dalla tastiera.
Mediante l'utilizzo di finestre grafiche di conferma o errore e di file di riepilogo sarà possibile monitorare l'esecuzione e avere un
riepilogo chiaro su ciò che è stato fatto.
La prima volta che si desidera avviare l'applicazione, questo deve essere fatto manualmente, per permettere allo stesso processo di essere impostato come operazione da eseguire automaticamente all'avvio del sistema.

## Preparazione

Per il corretto funzionamento di questa applicazione è essenziale inserire minuziosamente, su due righe separate, nel file `Info.txt`,
le due directory sorgente e destinazione. La prima, esistente, servirà per raccogliere tutti i file richiesti nel back-up; la seconda, non ancora creata, verrà utilizzata dal programma per depositare tutti i file copiati che, una volta terminata l'operazione, saranno reperibili nel il suddetto percorso.

## Comando iniziale

L'operazione di preparazione al back-up ha inizio dopo che viene ricevuto il primo comando con il mouse, ossia un rettangolo che tocca tutti gli angoli del monitor.

Per effettuare questo comando bisogna:  

- Cliccare sull'angolo in alto a sinistra, o il più vicino possibile (con un margine d'errore massimo di 20 pixel);  
- Mantenendo il pulsante cliccato, raggiungere uno ad uno tutti gli angoli dello schermo in senso antiorario, mantenendosi il più possibile sul bordo dello schermo (anche in questo caso il margine d'errore è di 20 pixel dal bordo);  
- Una volta raggiunto nuovamente l'angolo iniziale rilasciare il pulsante del mouse, sempre mantenendosi nel range sopra indicato (concludendo così il rettangolo).
- se l'operazione è andata a buon fine verrà emesso un doppio segnale acustico, atrimenti, al momento di un errore, ne sarà emesso uno singolo.

## Definizione eventuale formato

A seguito del comando iniziale, prima di avviare il back-up, è necessario decidere, attraverso un ulteriore comando apposito, che tipo di operazione eseguire. La scelta ammette due possibili opzioni:  

- ##### Back-up completo dell'intero contenuto della directory sorgente (incluse le sotto-cartelle)
  Dopo il comando di attivazione con il mouse, cliccare il tasto `a`, con o senza CAPSLOCK attivo, per selezionare la versione completa del back-up che effettuerà una copia dell'intero contenuto della directory sorgente verso la directory destinazione, mantenendo la stessa gerarchia e organizzazione iniziale. 
- ##### Back-up vincolato ad un certo formato dei file, specificato attraverso un preciso tasto della tastiera
  L'operazione di back-up sarà limitata solamente ai file con una specifica estensione, trascurando tutti gli altri (eventuali) file e sottocartelle. Alla fine dell'operazione, nella directory destinazione sarà presente un elenco dei soli file con il formato specificato, indipendentemente dalla struttura originale delle sottocartelle.</br>
  I comandi, con le relative estensioni, sono i seguenti:

    - `a`/`A` -> operazione completa;  
    - `b`/`B` -> operazione parziale con copia dei file `.csv`;  
    - `c`/`C` -> operazione parziale con copia dei file `.py`;  
    - `d`/`D` -> operazione parziale con copia dei file `.txt`;  
    - `e`/`E` -> operazione parziale con copia dei file `.java`;  
    - `f`/`F` -> operazione parziale con copia dei file `.npy`;  
    - `g`/`G` -> operazione parziale con copia dei file `.docx`;  
    - `h`/`H` -> operazione parziale con copia dei file `.css`;  
    - `i`/`I` -> operazione parziale con copia dei file `.js`;  
    - `j`/`J` -> operazione parziale con copia dei file `.html`;  
    - `k`/`K` -> operazione parziale con copia dei file `.mp3`;  
    - `l`/`L` -> operazione parziale con copia dei file `.jpg`;  
    - `m`/`M` -> operazione parziale con copia dei file `.png`;
    - `n`/`N` -> operazione parziale con copia dei file `.pdf`;  
    - `o`/`O` -> operazione parziale con copia dei file `.ppt`;
    - `p`/`P` -> operazione parziale con copia dei file `.xlsx`;

Qualora il tasto premuto non appartenga al precendente elenco, verrà emesso un segnale singolo di errore.

## Comando di conferma

Come ultimo comando prima dell'esecuzione del back-up è necessaria una conferma. Per questo comando è sufficiente tracciare una linea orizzontale da un estremo all'altro del monitor. Come nel caso del comando iniziale, è necessario:

- Cliccare in un punto qualsiasi lungo il bordo sinistro dello schermo, o il più vicino possibile (con un margine d'errore massimo di 20 pixel);  
- Mantenendo il pulsante cliccato, tracciare una linea orizzontale fino all'estremità opposta dello schermo, mantenendosi entro un range di 100 pixel.  
- Rilasciare il pulsante del mouse all'interno dell'area delimitata dai 20 pixel finali.
- Anche in questo caso, se l'operazione è andata a buon fine, verrà emesso un doppio segnale acustico; altrimenti, al momento dell'errore, ossia quando viene sforato il range consentito, ne verrà emesso uno singolo.

## Conclusione operazione

Una volta completato il comando di conferma verranno utilizzate le informazioni raccolte sul formato e quelle estratte dal file `Info.txt` per spostare i file. L'operazione ha inizio quando viene visualizzata la finestra di conferma e termina pochi secondi dopo. Alla conclusione sarà presente una nuova directory con all'interno tutti i file e/o le cartelle copiate, insieme ad un file di riepilogo contenente tutte le informazioni raccolte durante l'operazione, quali:  

- Data e ora dell'operazione;
- Descrizione dell'operazione di back-up eseguita; 
- Numero di file copiati;  
- Numero di cartelle copiate;  
- Dimensione totale dei file copiati;
- Durata dell'operazione (in micro-secondi).

Infine, un triplo segnale acustico determina il completamento dell'operazione.
