# Manuale utente

## Indice

- [Descrizione e scopo](#descrizione-e-scopo)
- [Preparazione](#preparazione)
- [Comando iniziale](#comando-iniziale)
- [Definizione versione e eventuale formato](#definizione-versione-e-eventuale-formato)
- [Comando di conferma](#comando-di-conferma)
- [Conclusione operazione](#conclusione-operazione)

## Descrizione e scopo

Quest'applicazione ha il compito di aiutare ad effettuare il back-up totale/parziale dei file da una dirctory ad un'altra, ed è specificatamente pensata per operare qualora lo schermo non sia agibile, attraverso comandi dal mouse e dalla tasiera.
Mediante l'utilizzo di finestre grafiche di conferma o errore e di file di riepilogo sarà possibile monitorare l'esecuzione e avere un
riepilogo chiaro su ciò che è stato fatto.

## Preparazione

Per il corretto funzionamento di questa applicazione è essenziale inserire minuziosamente, su due righe separate, nel file "Info.txt",
le due directory di sorgente e destinazione. La prima, esistente, servirà per raccogliere tutti i file richiesti nel back-up; la seconda, non ancora creata, verrà utilizzata dal programma per depositare tutti i file copiati che, una volta terminata l'operazione, saranno reperibili sotto il suddetto percorso.

## Comando iniziale

L'operazione di preparazione al back-up ha inizio dopo che viene ricevuto il primo comando con il mouse, ossia un rettangolo che tocca tutti gli angoli del monitor.

Per questo comando bisogna:  
--- Cliccare sull'angolo in alto a sinistra, o il più vicino possibile (massimo 20 pixel in ogni direzione);  
--- Mantenendo il pulsante cliccato, raggiungere uno ad uno tutti gli angoli dello schermo in senso antiorario, mantenendosi il più possibile sul bordo dello schermo (accettabilità entro 20 pixel dal bordo);  
--- Una volta raggiunto nuovamente l'angolo iniziale rilasciare il pulsante, sempre mantenendosi nel range sopra indicato (concludendo così il rettangolo).

## Definizione versione e eventuale formato

Una volta terminato il comando iniziale, prima di avviare il back-up è necessario decidere, attraverso un coomando apposito, che tipo di operazione eseguire. La scelta ammette due possibili operazioni:  
--- Back-up completo dell'intero contenuto della directory sorgente (incluse le sotto-cartelle);  
--- Back-up vincolato ad un certo formato dei file, specificato attraverso un preciso tasto della tastiera.

Nel primo caso sarà sufficiente, dopo il comando con il mouse, cliccare sul tasto 'a'/'A' per selezionare la versione completa del back-up che effettuerà una copia dell'intero contenuto della directory sorgene verso la directory destinazione, mantenendo la stessa gerarchia e organizzazione iniziale.  
Nel secondo caso l'operazione di back-up sarà limitata solamente ai file con un'estensione precisa, trascurando tutti gli altri (eventuali) file e le sottocartelle. Alla fine dell'operazione, nella directory destinazione sarà pressente un elenco dei soli file con il formato specificato, senza l'eventuale organizzazione iniziale in sottocartelle.  
'a'/'A' -> versione completa;  
'b'/'B' -> versione parziale con copia dei file ".csv";  
'c'/'C' -> versione parziale con copia dei file ".py";  
'd'/'D' -> versione parziale con copia dei file ".txt";  
'e'/'E' -> versione parziale con copia dei file ".java";  
'f'/'F' -> versione parziale con copia dei file ".npy";  
'g'/'G' -> versione parziale con copia dei file ".docx";  
'h'/'H' -> versione parziale con copia dei file ".css";  
'i'/'I' -> versione parziale con copia dei file ".js";  
'j'/'J' -> versione parziale con copia dei file ".html";  
'k'/'K' -> versione parziale con copia dei file ".mp3";  
'l'/'L' -> versione parziale con copia dei file ".jpg";  
'm'/'M' -> versione parziale con copia dei file ".png";

## Comando di conferma

Come ultimo comando prima dell'operazione è necessaria una conferma. Per questo comando è sufficiente tracciare una linea orizzontale da un estremo all'altro del monitor. Come nel caso del comando iniziale bisogna:

--- Cliccare in un punto qualsiasi lungo il bordo sinistro dello schermo, o il più vicino possibile (massimo 20 pixels dal bordo);  
--- Mantenendo il pulsante cliccato, tracciare una linea orizzontale fino alla estremità opposta dello schermo, mantenendosi entro un range di accettabilità di 100 pixel in entrambe le direzioni.  
--- Rilasciare il pulsante del mouse all'interno dell'area delimitata dai 20 pixel finali.

## Conclusione operazione

Una volta completato il comando di conferma verranno utilizzate e informazioni raccolte su versione e formato e quelle presenti nel file "Info.txt" per spostare i file. L'operazione ha inizio quando viene visualizzata la finestra di conferma e termine pochi secondi dopo; alla conclusione sarà presente una nuova directory con all'interno tutti i file e/o le cartelle copiate più un file di riepilogo contenente tutte le informazioni raccolte durante l'operazione quali:  
--- Dimensione totale dei file copiati;  
--- Numero di file copiati;  
--- Numero di cartelle copiate;  
--- Data e ora dell'operazione;  
--- Durata dell'operazione (in cicli di CPU).
