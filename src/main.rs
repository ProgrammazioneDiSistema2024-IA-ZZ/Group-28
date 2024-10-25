/* STATO: 
--- Back-Up v
--- Versioni 1 e 0 v
--- Riepilogo v
--- Finestra di conferma v
--- thread separato che stampa sul file v
--- Comandi v
--- Metterlo in background*/

use std::env;
use std::fs::{read_dir, create_dir, copy, OpenOptions, metadata};
use std::io::{BufRead, BufReader, Write};
use std::process::id;
use std::thread::{spawn, sleep};
use std::time::Duration;
use chrono::Local;
use druid::widget::{Flex, Label};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use druid::{WidgetExt, WindowDesc, Color, AppLauncher, FontDescriptor, FontFamily};
use rdev::{listen, Event, EventType, Key};
use cpu_time::ProcessTime;
use rodio::{source::SineWave, OutputStream, Sink, Source};
use auto_launch::AutoLaunchBuilder;

const PATH_FILE:&str="C:/Users/Alessandro/RustroverProjects/Group-28/src/Info.txt";

struct Mouse{                // RAPPRESENTA IL MOUSE DURANTE IL COMANDO
    n_fase: usize,               // IN CHE FASE SIAMO (1:ATTESA RETTANGOLO, 2:ATTESA COMANDO, 3:ATTESA CONFERMA)
    pos_x:i32,                     // COORDINATE ATTUALI
    pos_y:i32,
    pos_prec_y:i32,            // USATO PER IL MENO DI CONFERMA
    is_active_vec:[bool;4],        // ANGOLI TOCCATI O NO
    is_active:bool,               // COMANDO CONFERMA AVVIATO?
    range_rettangolo:i32,          // RANGE PER RETTANGOLO
    range_conferma:i32}                     // RANGE DELLA CONFERMA

impl Mouse{

    fn new()->Self{
        return Mouse{n_fase:1,pos_x:0,pos_y:0,pos_prec_y:0,is_active_vec:[false;4],is_active:false,range_conferma:100,range_rettangolo:20};}

    fn inizio_attesa(&mut self){                 // SE PREMI E SEI CIRCA LI' ATTIVO IL PRIMO ANGOLO
        if self.pos_x<=self.range_rettangolo && self.pos_y<=self.range_rettangolo{
            self.is_active_vec[0]=true;}
        return;}

    fn fine_attesa(&mut self){                     // SE LASCI E SEI CIRCA LI' PASSO ALLA SECONDA ATTESA
        if self.pos_x<=self.range_rettangolo && self.pos_y<=self.range_rettangolo && self.is_active_vec==[true,true,true,true]{
            suono_comando(2);
            self.n_fase=2;}
        self.is_active_vec=[false;4];         // AZZERO TUTTO
        return;}

    fn cambia_posizione_per_rettangolo(&mut self,x:i32,y:i32,height:i32,width:i32){       // SI MUOVE
        let mut valido=true;
        match self.is_active_vec{
            [true,false,false,false]=>{
                if x<=self.range_rettangolo && y>=height-self.range_rettangolo{     // TOCCA IN BASSO A SX
                    self.is_active_vec[1]=true;}
                else if x>self.range_rettangolo{                    // SFORA
                    valido=false;}}
            [true,true,false,false]=>{
                if x>=width-self.range_rettangolo && y>=height-self.range_rettangolo{     // TOCCA IN BASSO A DX
                    self.is_active_vec[2]=true;}
                else if y<height-self.range_rettangolo{         // SFORA
                    valido=false;}}
            [true,true,true,false]=>{
                if y<=self.range_rettangolo && x>=width-self.range_rettangolo{         // TOCCA IN ALTO A DX
                    self.is_active_vec[3]=true;}
                else if x<width-self.range_rettangolo{           //  SFORA
                    valido=false;}}
            [true,true,true,true]=>{
                if y>self.range_rettangolo{             // SFORA
                    valido=false;}}
            _=>{}}

        self.pos_x=x;
        self.pos_y=y;
        if valido==false{
            suono_comando(1);                      // SUONO DI ERRORE
            self.is_active_vec=[false;4];}                 // ERR=>AZZERA TUTTO
        return;}

    fn attivazione_conferma(&mut self){                // SI ATTIVA L'ATTESA DEL COMPLETAMENTO DEL COMANDO
        if self.pos_x<=self.range_rettangolo{          // SOLO SE SEI A SX
            self.pos_prec_y=self.pos_y;
            self.is_active=true;}                // DA ORA POS_PREC_Y E' L'ALTEZZA DEL MENO
        return;}

    fn disattivazione_conferma(&mut self,formato:Option<&str>,width:i32){   // AVVIA lA FUNZIONE O SPEGNE TUTTO
        if self.is_active && self.pos_x>=width-self.range_rettangolo{
            suono_comando(2);
            funzione_di_back_up(formato);         // CONTROLLO DELL'ALTEZZA NEL MOVIMENTO
            suono_comando(3);                                // FINITO
            self.n_fase=1;}
        self.is_active=false;          // AZZERA TUTTO
        return;}

    fn cambia_posizione_per_conferma(&mut self,x:i32,y:i32){               // IL MOUSE SI SPOSTA
        if (self.pos_y>self.pos_prec_y+self.range_conferma || self.pos_y<self.pos_prec_y-self.range_conferma) && self.is_active==true{
            suono_comando(1);                      // ERRORE
            self.is_active=false;}                       // SE SFORI SPENGO TUTTO
        self.pos_x=x;
        self.pos_y=y;
        return;}}

fn suono_comando(n_volte:i32){
    let (_stream,stream_handle)=OutputStream::try_default().unwrap();    // CREO L'AUDIO
    let frequenza=440.0;
    let sink=Sink::try_new(&stream_handle).unwrap();
    let mut sine_wave;
    
    for _ in 0..n_volte{
        sine_wave=SineWave::new(frequenza);            // GENERA IL SEGNALE SINUSOIDALE A F=440.0 HZ
        sink.append(sine_wave.take_duration(Duration::from_millis(200))); // DURATA A 200MS
        sink.sleep_until_end();               // PLAY
        sleep(Duration::from_millis(100));}       // PAUSA TRA I BIP
    return;}

fn copia_totale(sorgente:String, destinazione:String)->(u64, i32, i32){
    let dir_sorgente;                                                   // DIRECTORY SORGENTE
    let mut path_file;                                                   // PATH DEL FILE
    let mut path_destinazione;                                              // PATH IN CUI SCRIVERE
    let mut file_da_leggere;                                            // OGGETTO CORRENTE
    let mut nome_file;                                                   // NOME FILE
    let mut dim=0;                                                        // DIMENSIONE TOTALE DIRECTORY
    let mut n_file=0;                                                      // NUMERO-FILE
    let mut n_cartelle=0;                                                   // NUMERO DIRECTORY
    let mut com;                                                         // TUPLA DI COMODO

    match read_dir(sorgente.clone()){                                  // APRO LA DIRECTORY
        Err(_)=>{return (0,0,0);}
        Ok(dir)=>{dir_sorgente=dir;}}

    for nodo in dir_sorgente{                                    // PER OGNI FILE
        file_da_leggere=nodo.unwrap();
        nome_file=file_da_leggere.file_name().to_str().unwrap().to_string();    // PRENDO IL NOME
        path_file=format!("{}/{}",sorgente,nome_file);                          // PRENDO IL PATH
        path_destinazione=format!("{}/{}",destinazione,nome_file);                    // PATH SU CUI SCRIVERE

        if file_da_leggere.file_type().unwrap().is_file(){
            if copy(path_file.clone(),path_destinazione).is_err(){        // COPIO IL FILE
                panic!("Errore nel trasferimento del file '{}'",nome_file);}
            n_file=n_file+1;
            dim=dim+metadata(path_file).unwrap().len();}
        else{
            if create_dir(path_destinazione.clone()).is_err(){    // CREO LA DIRECTORY
                panic!("Errore nel trasferimento della directory '{}'",nome_file);}
            n_cartelle=n_cartelle+1;
            com=copia_totale(path_file,path_destinazione);           // DISCESA RICORSIVA
            dim=dim+com.0;
            n_file=n_file+com.1;
            n_cartelle=n_cartelle+com.2;}}                      // RISULTATI
    return (dim,n_file,n_cartelle);}

fn copia_file_specifici(sorgente:String,destinazione:String,formato:&str)->(u64,i32){
    let dir_sorgente;                                                   // DIRECTORY SORGENTE
    let mut path_file;                                                   // PATH DEL FILE
    let mut path_destinazione;                                              // PATH IN CUI SCRIVERE
    let mut file_da_leggere;                                            // OGGETTO CORRENTE
    let mut nome_file;                                                   // NOME FILE
    let mut dim=0;                                                         // DIMENSIONE DIRECTORY
    let mut n_file=0;                                                      // NUMERO-FILE
    let mut com;                                                               // TUPLA DI COMODO

    match read_dir(sorgente.clone()){                                  // APRO LA DIRECTORY
        Err(_)=>{return (0,0);}
        Ok(dir)=>{dir_sorgente=dir;}}

    for nodo in dir_sorgente{                                    // PER OGNI FILE
        file_da_leggere=nodo.unwrap();
        nome_file=file_da_leggere.file_name().to_str().unwrap().to_string();    // PRENDO IL NOME
        path_file=format!("{}/{}",sorgente,nome_file);                          // PRENDO IL PATH
        path_destinazione=format!("{}/{}",destinazione,nome_file);                    // PATH SU CUI SCRIVERE

        if file_da_leggere.file_type().unwrap().is_file() && file_da_leggere.path().extension()==Some(formato.as_ref()){
            if copy(path_file.clone(),path_destinazione).is_err(){        // COPIO IL FILE
                panic!("Errore nel trasferimento del file '{}'",nome_file);}
            n_file=n_file+1;
            dim=dim+metadata(path_file).unwrap().len();}
        else if file_da_leggere.file_type().unwrap().is_dir(){                   // LO METTO COMUNQUE IN ELIF
            com=copia_file_specifici(path_file,destinazione.clone(),formato);           // DISCESA RICORSIVA
            dim=dim+com.0;
            n_file=n_file+com.1;}}                     // RISULTATI
    return (dim,n_file);}

fn leggi_info()->(String,String){
    let file=OpenOptions::new().read(true).open(PATH_FILE).expect("File sorgente non trovato");
    let mut linee=BufReader::new(file).lines();                  // LEGGO LE DUE LINEE
    return (linee.next().unwrap().unwrap(),linee.next().unwrap().unwrap());}

fn crea_riepilogo(src:String,dim:u64,n_file:i32,n_cartelle:i32,tempo:u128,operazione:String){
    let ora=Local::now();                   // LEGGO 'ORA
    let msg=format!("Operazione completata il {}.\n\nInformazioni aggiuntive:\n--{}\n--Numero file copiati: {}\n--\
                           Numero cartelle copiate: {}\n--Dimensione totale: {} bytes\n--\
                           Tempo di CPU: {} micro-secondi",ora,operazione,n_file,n_cartelle,dim,tempo);    // MSG
    let path=format!("{}/Riepilogo.txt",src);                // CREO IL PATH E IL FILE
    let mut file =OpenOptions::new().create(true).write(true).open(path).expect("Impossibile creare il file");
    if file.write_all(msg.as_bytes()).is_err(){
        crea_finestra_errore("Errore nel riepilogo operazione");}               // ERRORE DI SCRITTURA
    return;}

fn scrivi_ogni_tanto(){                              // SCRIVE MESSAGGI A INTERVALLI REGOLARI
    let mut system=System::new_all();
    let mut file =OpenOptions::new().append(true).open(PATH_FILE).expect("File non trovato");
    let mut msg;
    let mut ora;                                           // PER OTTENERE L'ORA IN FORMATO LEGGIBILE
    let mut uso;
    let pid=Pid::from_u32(id());
    loop{
        sleep(Duration::from_secs(120));                           // ATTENDI...
        system.refresh_all();                                            // LEGGE I DATI DAL SISTEMA
        ora=Local::now();
        uso=system.process(pid).expect("Processo non trovato").cpu_usage();        // PRENDO LE INFORMAZIONI
        msg=format!("\nConsumo di CPU dal processo {} fino all'istante {}: {}%",pid,ora,uso);
        if file.write_all(msg.as_bytes()).is_err(){                // ERRORE DI SCRITTURA
            crea_finestra_errore("Errore nella scrittura sul file");
            break;}}
    return;}

fn funzione_di_back_up(formato:Option<&str>){
    let (directory_sorgente,directory_destinazione)=leggi_info();        // LETTURA DELLE DIRECTORY
    let dim_totale;                                                     // DIMENSIONE TOTALE DEI FILE SPOSTATI
    let numero_file;                                                    // CONTATORE DEI FILE
    let mut numero_cartelle=0;                                               // CONTATORE SOTTO-DIRECTORY
    let (src_clone,dest_clone)=(directory_sorgente.clone(),directory_destinazione.clone());
    let msg;                                                 // COSA SCRIVO COME OPERAZIONE NEL FILE DI RIEPILOGO
    let time;

    if read_dir(directory_sorgente.clone()).is_err(){            // VERIFICA SE ESISTE LA DIRECTORY SORGENTE
        spawn(|| crea_finestra_errore("\nDirectory sorgente non trovata"));
        return;}
    if create_dir(directory_destinazione.clone()).is_err(){       // CREO LA DIRECTORY OBIETTIVO
        spawn(|| crea_finestra_errore("\nImpossibile creare la directory di destinazione"));
        return;}

    spawn(|| crea_finestra_successo(src_clone,dest_clone));      // LANCIA LA FINESTRA

    time=ProcessTime::now();       // PARTE IL CRONOMETRO (CRONOMETRA IL PROCESSO -> PIU' VICINO AL BACKUP PARTE, PIU' ACCURATO E')
    if formato.is_none(){
        (dim_totale,numero_file,numero_cartelle)=copia_totale(directory_sorgente,directory_destinazione.clone()); // COPIA TUTTO
        msg="Descrizione: copia di tutti i file/directory".to_string();}
    else{
        (dim_totale,numero_file)=copia_file_specifici(directory_sorgente,directory_destinazione.clone(),formato.unwrap()); // SOLO I RICHIESTI
        msg=format!("Descrizione: copia dei soli file con estensione '{}'",formato.unwrap());}

    crea_riepilogo(directory_destinazione,dim_totale,numero_file,numero_cartelle,time.elapsed().as_micros(),msg); // CREA IL FILE DI RIEPILOGO
    return;}

fn crea_finestra_successo(src:String,dest:String){                         // CREA LA FINESTRA DI CONFERMA
    let ora=Local::now();
    let text=format!("\n\nBack-up in corso...\n\n\nSorgente: '{}'\n\nDestinazione: '{}'\n\n\nOra: {}",src,dest,ora);// TESTO DA SCRIVERE
    let label=Label::new(text).with_text_color(Color::WHITE);           // BLOCCO INTERNO CON IL TESTO
    let flex=Flex::column().with_child(label).background(Color::GREEN);        // FLEX DI ALLINEAMENTO ORIZZONTALE
    let window=WindowDesc::new(flex).window_size((600.0,300.0));    // CREO LA FINESTRA DI CONFERMA
    if AppLauncher::with_window(window).launch(()).is_err(){
        panic!("Errore nel lancio dell'applicazione");}                                // ERRORE NON RECUPERABILE
    return;}

fn crea_finestra_errore(msg: &str){                         // CREA LA FINESTRA DI ERRORE
    let font=FontDescriptor::new(FontFamily::SANS_SERIF).with_size(30.0);   // FONT DEL TESTO
    let label=Label::new(msg).with_text_color(Color::WHITE).with_font(font);           // BLOCCO INTERNO CON IL TESTO
    let flex=Flex::column().with_child(label).background(Color::RED);        // FLEX DI ALLINEAMENTO ORIZZONTALE
    let window=WindowDesc::new(flex).window_size((650.0,200.0));    // CREO LA FINESTRA DI ERRORE
    if AppLauncher::with_window(window).launch(()).is_err(){
        panic!("Errore nel lancio dell'applicazione");}                                // ERRORE NON RECUPERABILE
    return;}

fn ottieni_dimensioni()->(u32,u32){                     // SOLO MOMENTANEO
    return (1920,1080);}

fn inizio_operazione(){
    let (width,height)=ottieni_dimensioni();
    let mut mouse=Mouse::new();                                 // RAPPRESENTA IL MOUSE
    let mut formato=None;                                            // VARIABILI PER LA FUNZIONE DI BACK UP

    let callback_comando=move|event:Event|{
        if mouse.n_fase==1{            // CASO DEL COMANDO DEL RETTANGOLO
            match event.event_type{
                EventType::MouseMove{x,y}=>mouse.cambia_posizione_per_rettangolo(x as i32,y as i32,height as i32,width as i32),
                EventType::ButtonPress(_)=>mouse.inizio_attesa(),                                 // CLICK->ATTIVO LA LETTURA DEL RETTANGOLO
                EventType::ButtonRelease(_)=>mouse.fine_attesa(),                 // SE COMPLETI SI PASSA ALLA FFASE 2
                _=>{}}}
        else if mouse.n_fase==2{                               // CASO DEL COMANDO DA TASTIERA
            if let EventType::KeyPress(pulsante)=event.event_type{
                mouse.n_fase=3;
                match pulsante{                                                       // DECIDI QUALE VERSIONE
                    Key::KeyA=>formato=None,          // SE 'a'/'A' -> TOTALE
                    Key::KeyB=>formato=Some("csv"),   // SE 'b'/'B' -> SOLO I .csv
                    Key::KeyC=>formato=Some("py"),    // SE 'c'/'C' -> SOLO I .py
                    Key::KeyD=>formato=Some("txt"),   // SE 'd'/'D' -> SOLO I .txt
                    Key::KeyE=>formato=Some("java"),  // SE 'e'/'E' -> SOLO I .java
                    Key::KeyF=>formato=Some("npy"),   // SE 'f'/'F' -> SOLO I .npy
                    Key::KeyG=>formato=Some("docx"),  // SE 'g'/'G' -> SOLO I .docx
                    Key::KeyH=>formato=Some("css"),   // SE 'h'/'H' -> SOLO I .css
                    Key::KeyI=>formato=Some("js"),    // SE 'i'/'I' -> SOLO I .js
                    Key::KeyJ=>formato=Some("html"),  // SE 'j'/'J' -> SOLO I .html
                    Key::KeyK=>formato=Some("mp3"),   // SE 'k'/'K' -> SOLO I .mp3
                    Key::KeyL=>formato=Some("jpg"),   // SE 'l'/'L' -> SOLO I .jpg
                    Key::KeyM=>formato=Some("png"),   // SE 'm'/'M' -> SOLO I .png
                    Key::KeyN=>formato=Some("pdf"),   // SE 'n'/'N' -> SOLO I .pdf
                    Key::KeyO=>formato=Some("ppt"),  // SE 'o'/'O' -> SOLO I .ppt
                    Key::KeyP=>formato=Some("xlsx"),  // SE 'p'/'P' -> SOLO I .xlsx
                    _=>{mouse.n_fase=1;suono_comando(1);}}}}        // ERRORE
        else{
            match event.event_type{
                EventType::MouseMove{x,y}=>mouse.cambia_posizione_per_conferma(x as i32,y as i32),             // SE TI MUOVI
                EventType::ButtonPress(_)=>mouse.attivazione_conferma(),                  // CLICK->ATTIVO (FORSE) IL COMANDO
                EventType::ButtonRelease(_)=>mouse.disattivazione_conferma(formato,width as i32),  // SE COMPLETI IL - AVVIO IL BACKUP
                EventType::KeyPress(_)=>mouse.n_fase=1,                                     // ESCAPE
                _=>{}}}};

    spawn(|| scrivi_ogni_tanto());                                                // MESSAGGI SUL FILE

    if listen(callback_comando).is_err(){                          // ATTESA
        crea_finestra_errore("Impossibile inizializzare l'operazione");}
    return;}

fn main(){
    let path=env::current_exe().unwrap().as_os_str().to_str().unwrap().to_string();            // PERCORSO DELL'ESEGUIBILE
    let auto_launch=AutoLaunchBuilder::new().set_app_name("Group28").set_app_path(path.as_str()).build()
        .expect("Errore nell'avvio app");                   // CREO L'ISTANZA E CARICO L'APPLICAZIONE CORRENTE
    
    match auto_launch.is_enabled(){                // E' GIA' AVVIATO?
        Ok(bool)=>{
            if bool==true{
                inizio_operazione();}            // SE E' SI' L'AVVIO
            else{
                auto_launch.enable().expect("Errore nell'abilitazione");}}     // LA ABILITO
        Err(_)=>println!("Errore nell'abilitare l'auto-launch")}                // ERRORE
    return;}
