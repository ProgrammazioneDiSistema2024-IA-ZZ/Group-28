/* STATO: 
--- Back-Up v
--- Versioni 1 e 0 v
--- Riepilogo v
--- Finestra di conferma v
--- thread separato che stampa sul file v
--- Comandi v
--- Metterlo in background*/

use std::fs::{read_dir, create_dir, copy, OpenOptions, metadata};
use std::io::{BufRead, BufReader, Write};
use std::process::id;
use std::thread::{spawn, sleep};
use std::time::{Duration, SystemTime};
use chrono::{DateTime,Utc};
use druid::widget::{Flex, Label};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use druid::{WidgetExt, WindowDesc, Color, AppLauncher, FontDescriptor, FontFamily};
use rdev::{listen, Event, EventType};
use cpu_time::ProcessTime;

const HEIGHT:f64=1080.0;
const WIDTH:f64=1920.0;

struct Mouse{                // RAPPRESENTA IL MOUSE DURANTE IL COMANDO
    pos_x:f64,                     // COORDINATE ATTUALI
    pos_y:f64,
    pos_prec_y: f64,            // USATO PER IL MENO DI CONFERMA
    is_active_vec:[bool;4],        // ANGOLI TOCCATI O NO
    is_active:bool,               // COMANDO CONFERMA AVVATO?
    range_rettangolo:f64,          // RANGE PER RETTANGOLO
    range_conferma: f64}                     // RANGE DI ACCETTABILITA'

impl Mouse{

    fn new()->Self{
        return Mouse{pos_x:0.0,pos_y:0.0,pos_prec_y:0.0,is_active_vec:[false;4],is_active:false,range_conferma:100.0,range_rettangolo:20.0};}

    fn inizio_attesa(&mut self){                 // SE PREMI E SEI CIRCA LI' ATTIVO IL RETTANGOLO
        if self.pos_x<=self.range_rettangolo && self.pos_y<=self.range_rettangolo{
            self.is_active_vec[0]=true;}
        return;}

    fn fine_attesa(&mut self)->i32{                     // SE LASCI E SEI CIRCA LI' PASSO ALLA SECONDA ATTESA
        let mut val=1;
        if self.pos_x<=self.range_rettangolo && self.pos_y<=self.range_rettangolo && self.is_active_vec==[true,true,true,true]{
            val=2;}
        self.is_active_vec=[false;4];
        return val;}

    fn cambia_posizione_per_rettangolo(&mut self,x:f64,y:f64){       // SI MUOVE
        let mut valido=1;
        match self.is_active_vec{
            [true,false,false,false]=>{
                if x<=self.range_rettangolo && y>=HEIGHT-self.range_rettangolo{     // TOCCA IN BASSO A SX
                    self.is_active_vec[1]=true;}
                else if x>self.range_rettangolo{                    // SFORA A DX
                    valido=0;}}
            [true,true,false,false]=>{
                if x>=WIDTH-self.range_rettangolo && y>=HEIGHT-self.range_rettangolo{     // TOCCA IN BASSO A DX
                    self.is_active_vec[2]=true;}
                else if y<HEIGHT-self.range_rettangolo{         // SFORA
                    valido=0;}}
            [true,true,true,false]=>{
                if y<=self.range_rettangolo && x>=WIDTH-self.range_rettangolo{         // TOCCA IN ALTO A DX
                    self.is_active_vec[3]=true;}
                else if x<WIDTH-self.range_rettangolo{           //  SFORA
                    valido=0;}}
            [true,true,true,true]=>{
                if y>self.range_rettangolo{             // SFORA
                    valido=0;}}
            _=>{}}

        self.pos_x=x;
        self.pos_y=y;
        if valido==0{
            self.is_active_vec=[false;4];}                 // ERR=>AZZERA TUTTO
        println!("{}  {}  {:?}",x,y,self.is_active_vec);
        return;}

    fn attivazione_conferma(&mut self){                               // SI ATTIVA L'ATTESA DEL COMPLETAMENTO DEL COMANDO
        if self.pos_x<=self.range_rettangolo{                        // SOLO SE SEI A SX
            self.pos_prec_y=self.pos_y;
            self.is_active=true;}                                         // DA ORA POS_PREC_Y E' LA POS DEL MENO
        return;}

    fn disattivazione_conferma(&mut self)->i32{                        // AVVIA lA FUNZIONE O SPEGNE TUTTO
        let mut risposta=2;
        if self.pos_y<=self.pos_prec_y+self.range_conferma && self.pos_y>=self.pos_prec_y-self.range_conferma && self.is_active
            && self.pos_x>=WIDTH-self.range_rettangolo{
            funzione_di_back_up();
            risposta=1;}
        self.is_active=false;
        return risposta;}                 // RISPOSTA RAPPRESENTA LO STATO IN CUI TORNARE (1: ATTENDO UN RETTANGOLO, 2: CONFERMA MANCATA)

    fn cambia_posizione_per_conferma(&mut self,x:f64,y:f64){               // IL MOUSE SI SPOSTA
        if self.pos_y>self.pos_prec_y+self.range_conferma || self.pos_y<self.pos_prec_y-self.range_conferma{
            self.is_active=false;}                       // SOLO IN AVANTI, UNA VOLTA AVVIATO
        self.pos_x=x;
        self.pos_y=y;
        println!("{}   {}    {}",x,y,self.is_active);
        return;}}

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
            if let Err(_)=copy(path_file.clone(),path_destinazione){        // COPIO IL FILE
                panic!("Errore nel trasferimento del file '{}'",nome_file);}
            n_file=n_file+1;
            dim=dim+metadata(path_file).unwrap().len();}
        else if file_da_leggere.file_type().unwrap().is_dir(){                   // LO METTO COMUNQUE IN ELIF
            com=copia_file_specifici(path_file,destinazione.clone(),formato);           // DISCESA RICORSIVA
            dim=dim+com.0;
            n_file=n_file+com.1;}}                     // RISULTATI
    return (dim,n_file);}

fn leggi_info()->(String,String){
    let file=OpenOptions::new().read(true).open("src/Info.txt").expect("File sorgente non trovato");
    let mut linee=BufReader::new(file).lines();                  // LEGGO LE DUE LINEE
    return (linee.next().unwrap().unwrap(),linee.next().unwrap().unwrap());}

fn crea_riepilogo(src:String,dim:u64,n_file:i32,n_cartelle:i32,tempo:f64){
    let ora: DateTime<Utc>=SystemTime::now().into();
    let msg=format!("Operazione completata il {}.\nInformazioni aggiuntive:\n--Numero file copiati: {}\n--\
                           Numero cartelle copiate: {}\n--Dimensione totale: {} bytes\n--\
                           Tempo di CPU: {} secondi",ora,n_file,n_cartelle,dim,tempo);
    let path=format!("{}/Riepilogo.txt",src);
    let mut file =OpenOptions::new().create(true).write(true).open(path).expect("Impossibile creare il file");
    if file.write_all(msg.as_bytes()).is_err(){
        panic!("Errore nel riepilogo operazione");}
    return;}

fn scrivi_ogni_tanto(){                              // SCRIVE MESSAGGI A INTERVALLI REGOLARI
    let mut system=System::new_all();
    let mut file =OpenOptions::new().append(true).open("src/Info.txt").expect("File non trovato");
    let mut msg;
    let mut ora: DateTime<Utc>;                                            // PER OTTENERE L'ORA IN FORMATO LEGGIBILE
    let mut uso;
    let pid=Pid::from_u32(id());
    loop{
        sleep(Duration::from_secs(120));                           // ATTENDI...
        system.refresh_all();                                            // LEGGE I DATI DAL SISTEMA
        ora=SystemTime::now().into();
        uso=system.process(pid).expect("Processo non trovato").cpu_usage();
        msg=format!("\nConsumo di CPU dal processo {} fino all'istante {:?}: {}%",pid,ora,uso);
        if file.write_all(msg.as_bytes()).is_err(){
            break;}}
    return;}

fn funzione_di_back_up(){
    let (directory_sorgente,directory_destinazione)=leggi_info();        // LETTURA DELLE DIRECTORY
    let versione=0;                                          // VERSIONE DELL'OPERAZIONE
    let dim_totale;                                                     // DIMENSIONE TOTALE DEI FILE SPOSTATI
    let numero_file;                                                    // CONTATORE DEI FILE
    let mut numero_cartelle=0;                                               // CONTATORE SOTTO-DIRECTORY
    let (src_clone,dest_clone)=(directory_sorgente.clone(),directory_destinazione.clone());
    let time=ProcessTime::now();                                         // PARTE IL CRONOMETRO

    if read_dir(directory_sorgente.clone()).is_err(){            // VERIFICA SE ESISTE LA DIRECTORY SORGENTE
        println!("Directory '{}' non trovata",directory_sorgente.clone());
        spawn(|| crea_finestra_errore("\nDirectory sorgente non trovata"));
        return;}
    if create_dir(directory_destinazione.clone()).is_err(){       // CREO LA DIRECTORY OBIETTIVO
        println!("Errore nella creazione di '{}'",directory_destinazione.clone());
        spawn(|| crea_finestra_errore("\nImpossibile creare la directory di destinazione"));
        return;}

    spawn(|| crea_finestra_successo(src_clone,dest_clone));      // LANCIA LA FINESTRA

    if versione==0{
        (dim_totale,numero_file,numero_cartelle)=copia_totale(directory_sorgente,directory_destinazione.clone());} // COPIA TUTTO
    else{
        (dim_totale,numero_file)=copia_file_specifici(directory_sorgente,directory_destinazione.clone(),"csv");} // SOLO I "csv"

    crea_riepilogo(directory_destinazione,dim_totale,numero_file,numero_cartelle,time.elapsed().as_secs_f64()); // CREA IL FILE DI RIEPILOGO
    return;}

fn crea_finestra_successo(src:String,dest:String){                         // CREA LA FINESTRA DI CONFERMA
    let ora:DateTime<Utc>=SystemTime::now().into();
    let text =format!("\n\nBack-up in corso...\n\n\nSorgente: '{}'\n\nDestinazione: '{}'\n\n\nOra: {}",src,dest,ora);// TESTO DA SCRIVERE
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

fn main(){
    let mut mouse=Mouse::new();                                 // RAPPRESENTA IL MOUSE
    let mut attesa_comando=1;                                 // RAPPRESNTA IL COMANDO ATTESO (1:RETTANGOLO,2:MENO)

    let callback_comando=move|event:Event|{
        if attesa_comando==1{            // CASO DEL COMANDO DEL RETTANGOLO
            match event.event_type{
                EventType::MouseMove{x,y}=>mouse.cambia_posizione_per_rettangolo(x,y),    // SE TI MUOVI
                EventType::ButtonPress(_)=>mouse.inizio_attesa(),                                 // CLICK->ATTIVO LA LETTURA DEL RETTANGOLO
                EventType::ButtonRelease(_)=>attesa_comando=mouse.fine_attesa(),                 // SE COMPLETI SI PASSA ALLA FFASE 2
                _=>{}}}
        else{                               // CASO DEL COMANDO DI CONFERMA
            match event.event_type{
                EventType::MouseMove{x,y}=>mouse.cambia_posizione_per_conferma(x,y),             // SE TI MUOVI
                EventType::ButtonPress(_)=>mouse.attivazione_conferma(),                  // CLICK->ATTIVO (FORSE) IL COMANDO
                EventType::ButtonRelease(_)=>attesa_comando=mouse.disattivazione_conferma(),  // SE COMPLETI IL - AVVIO IL BACKUP
                EventType::KeyPress(_)=>attesa_comando=1,                                     // ESCAPE
                _=>{}}}};

    spawn(|| scrivi_ogni_tanto());                                                // MESSAGGI SUL FILE

    if let Err(msg)=listen(callback_comando){                          // ATTESA
        println!("Errore: {:?}",msg);}
    return;}
