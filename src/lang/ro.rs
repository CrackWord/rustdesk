lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stare"),
        ("Your Desktop", "Desktopul tău"),
        ("desk_tip", "Desktopul tău poate fi accesat folosind ID-ul și parola de mai jos."),
        ("Password", "Parolă"),
        ("Ready", "Pregătit"),
        ("Established", "Stabilit"),
        ("connecting_status", "În curs de conectare la rețeaua StarDesk..."),
        ("Enable service", "Activează serviciul"),
        ("Start service", "Pornește serviciul"),
        ("Service is running", "Serviciul rulează..."),
        ("Service is not running", "Serviciul nu funcționează"),
        ("not_ready_status", "Nepregătit. Verifică conexiunea la rețea."),
        ("Control Remote Desktop", "Controlează desktopul la distanță"),
        ("Transfer file", "Transferă fișiere"),
        ("Connect", "Conectează-te"),
        ("Recent sessions", "Sesiuni recente"),
        ("Address book", "Agendă"),
        ("Confirmation", "Confirmare"),
        ("TCP tunneling", "Tunel TCP"),
        ("Remove", "Elimină"),
        ("Refresh random password", "Actualizează parola aleatorie"),
        ("Set your own password", "Setează propria parolă"),
        ("Enable keyboard/mouse", "Activează control tastatură/mouse"),
        ("Enable clipboard", "Activează clipboard"),
        ("Enable file transfer", "Activează transferul de fișiere"),
        ("Enable TCP tunneling", "Activează tunelul TCP"),
        ("IP Whitelisting", "Listă de IP-uri autorizate"),
        ("ID/Relay Server", "Server de ID/retransmisie"),
        ("Import server config", "Importă configurație server"),
        ("Export Server Config", "Exportă configurație server"),
        ("Import server configuration successfully", "Configurație server importată cu succes"),
        ("Export server configuration successfully", "Configurație server exportată cu succes"),
        ("Invalid server configuration", "Configurație server nevalidă"),
        ("Clipboard is empty", "Clipboard gol"),
        ("Stop service", "Oprește serviciul"),
        ("Change ID", "Schimbă ID"),
        ("Your new ID", "Noul tău ID"),
        ("length %min% to %max%", "lungime între %min% și %max%"),
        ("starts with a letter", "începe cu o literă"),
        ("allowed characters", "caractere permise"),
        ("id_change_tip", "Pot fi utilizate doar caractere a-z, A-Z, 0-9, _ (bară jos). Primul caracter trebuie să fie a-z, A-Z. Lungimea trebuie să fie între 6 și 16 caractere."),
        ("Website", "Site web"),
        ("About", "Despre"),
        ("Slogan_tip", "Făcut din inimă în lumea aceasta haotică!"),
        ("Privacy Statement", "Politică de confidențialitate"),
        ("Mute", "Dezactivează sunet"),
        ("Build Date", "Dată build"),
        ("Version", "Versiune"),
        ("Home", "Acasă"),
        ("Audio Input", "Intrări audio"),
        ("Enhancements", "Îmbunătățiri"),
        ("Hardware Codec", "Codec hardware"),
        ("Adaptive bitrate", "Rată de biți adaptabilă"),
        ("ID Server", "Server de ID"),
        ("Relay Server", "Server de retransmisie"),
        ("API Server", "Server API"),
        ("invalid_http", "Trebuie să înceapă cu http:// sau https://"),
        ("Invalid IP", "IP nevalid"),
        ("Invalid format", "Format nevalid"),
        ("server_not_support", "Încă nu este compatibil cu serverul"),
        ("Not available", "Indisponibil"),
        ("Too frequent", "Modificat prea frecvent"),
        ("Cancel", "Anulează"),
        ("Skip", "Omite"),
        ("Close", "Închide"),
        ("Retry", "Reîncearcă"),
        ("OK", "OK"),
        ("Password Required", "Parolă necesară"),
        ("Please enter your password", "Introdu parola"),
        ("Remember password", "Memorează parola"),
        ("Wrong Password", "Parolă incorectă"),
        ("Do you want to enter again?", "Vrei să intri din nou?"),
        ("Connection Error", "Eroare de conexiune"),
        ("Error", "Eroare"),
        ("Reset by the peer", "Conexiunea a fost închisă de dispozitivul pereche"),
        ("Connecting...", "Se conectează..."),
        ("Connection in progress. Please wait.", "Conectare în curs. Te rugăm așteaptă."),
        ("Please try 1 minute later", "Reîncearcă într-un minut"),
        ("Login Error", "Eroare de autentificare"),
        ("Successful", "Succes"),
        ("Connected, waiting for image...", "Conectat, se așteaptă transmiterea imaginii..."),
        ("Name", "Denumire"),
        ("Type", "Tip"),
        ("Modified", "Modificat"),
        ("Size", "Dimensiune"),
        ("Show Hidden Files", "Afișează fișiere ascunse"),
        ("Receive", "Acceptă"),
        ("Send", "Trimite"),
        ("Refresh File", "Actualizează fișier"),
        ("Local", "Local"),
        ("Remote", "La distanță"),
        ("Remote Computer", "Computer la distanță"),
        ("Local Computer", "Computer local"),
        ("Confirm Delete", "Confirmă ștergerea"),
        ("Delete", "Șterge"),
        ("Properties", "Caracteristici"),
        ("Multi Select", "Alegere multiplă"),
        ("Select All", "Selectează tot"),
        ("Unselect All", "Deselectează tot"),
        ("Empty Directory", "Director gol"),
        ("Not an empty directory", "Directorul nu este gol"),
        ("Are you sure you want to delete this file?", "Sigur vrei să ștergi acest fișier?"),
        ("Are you sure you want to delete this empty directory?", "Sigur vrei să ștergi acest director gol?"),
        ("Are you sure you want to delete the file of this directory?", "Sigur vrei să ștergi fișierul din acest director?"),
        ("Do this for all conflicts", "Aplică la toate conflictele"),
        ("This is irreversible!", "Această acțiune este ireversibilă!"),
        ("Deleting", "În curs de ștergere..."),
        ("files", "fișier"),
        ("Waiting", "În așteptare..."),
        ("Finished", "Finalizat"),
        ("Speed", "Viteză"),
        ("Custom Image Quality", "Setează calitatea imaginii"),
        ("Privacy mode", "Mod privat"),
        ("Block user input", "Blochează intervenție utilizator"),
        ("Unblock user input", "Deblochează intervenție utilizator"),
        ("Adjust Window", "Ajustează fereastra"),
        ("Original", "Dimensiune originală"),
        ("Shrink", "Micșorează"),
        ("Stretch", "Extinde"),
        ("Scrollbar", "Bară de derulare"),
        ("ScrollAuto", "Derulare automată"),
        ("Good image quality", "Calitate bună a imaginii"),
        ("Balanced", "Calitate normală a imaginii"),
        ("Optimize reaction time", "Timp de reacție optimizat"),
        ("Custom", "Personalizat"),
        ("Show remote cursor", "Afișează cursor la distanță"),
        ("Show quality monitor", "Afișează detalii despre conexiune"),
        ("Disable clipboard", "Dezactivează clipboard"),
        ("Lock after session end", "Blochează după deconectare"),
        ("Insert", "Introdu"),
        ("Insert Lock", "Blochează computer"),
        ("Refresh", "Reîmprospătează"),
        ("ID does not exist", "ID neexistent"),
        ("Failed to connect to rendezvous server", "Conectare la server rendezvous eșuată"),
        ("Please try later", "Încearcă mai târziu"),
        ("Remote desktop is offline", "Desktopul la distanță este offline"),
        ("Key mismatch", "Nepotrivire cheie"),
        ("Timeout", "Conexiune expirată"),
        ("Failed to connect to relay server", "Conectare la server de retransmisie eșuată"),
        ("Failed to connect via rendezvous server", "Conectare prin intermediul serverului rendezvous eșuată"),
        ("Failed to connect via relay server", "Conectare prin intermediul serverului de retransmisie eșuată"),
        ("Failed to make direct connection to remote desktop", "Imposibil de stabilit o conexiune directă cu desktopul la distanță"),
        ("Set Password", "Setează parola"),
        ("OS Password", "Parolă sistem"),
        ("install_tip", "Din cauza restricțiilor CCU, este posibil ca StarDesk să nu funcționeze corespunzător. Pentru a evita acest lucru, dă clic pe butonul de mai jos pentru a instala StarDesk."),
        ("Click to upgrade", "Dă clic pentru a face upgrade"),
        ("Click to download", "Dă clic pentru a descărca"),
        ("Click to update", "Dă clic pentru a actualiza"),
        ("Configure", "Configurează"),
        ("config_acc", "Pentru a controla desktopul la distanță, trebuie să permiți StarDesk acces la setările de Accesibilitate."),
        ("config_screen", "Pentru a controla desktopul la distanță, trebuie să permiți StarDesk acces la setările de Înregistrare ecran."),
        ("Installing ...", "Se instalează..."),
        ("Install", "Instalează"),
        ("Installation", "Instalare"),
        ("Installation Path", "Cale de instalare"),
        ("Create start menu shortcuts", "Creează comenzi rapide în meniul Start"),
        ("Create desktop icon", "Creează pictogramă pe desktop"),
        ("agreement_tip", "Începerea procesului de instalare înseamnă acceptarea acordului de licență."),
        ("Accept and Install", "Acceptă și instalează"),
        ("End-user license agreement", "Acord de licență pentru utilizatorul final"),
        ("Generating ...", "Se generează..."),
        ("Your installation is lower version.", "Versiunea instalată este una inferioară."),
        ("not_close_tcp_tip", "Nu închide această fereastră în timp ce folosești tunelul"),
        ("Listening ...", "În așteptarea conexiunii tunel..."),
        ("Remote Host", "Gazdă la distanță"),
        ("Remote Port", "Port la distanță"),
        ("Action", "Acțiune"),
        ("Add", "Adaugă"),
        ("Local Port", "Port local"),
        ("Local Address", "Adresă locală"),
        ("Change Local Port", "Modifică port local"),
        ("setup_server_tip", "Pentru o conexiune mai rapidă, îți poți configura propriul server."),
        ("Too short, at least 6 characters.", "Prea scurtă; trebuie cel puțin 6 caractere."),
        ("The confirmation is not identical.", "Cele două intrări nu corespund."),
        ("Permissions", "Permisiuni"),
        ("Accept", "Acceptă"),
        ("Dismiss", "Respinge"),
        ("Disconnect", "Deconectează-te"),
        ("Enable file copy and paste", "Permite copierea și lipirea fișierelor"),
        ("Connected", "Conectat"),
        ("Direct and encrypted connection", "Conexiune directă criptată"),
        ("Relayed and encrypted connection", "Conexiune retransmisă criptată"),
        ("Direct and unencrypted connection", "Conexiune directă necriptată"),
        ("Relayed and unencrypted connection", "Conexiune retransmisă necriptată"),
        ("Enter Remote ID", "Introdu ID-ul dispozitivului la distanță"),
        ("Enter your password", "Introdu parola"),
        ("Logging in...", "Se conectează..."),
        ("Enable RDP session sharing", "Activează partajarea sesiunii RDP"),
        ("Auto Login", "Conectare automată (validă doar dacă opțiunea Blocare după deconectare este selectată)"),
        ("Enable direct IP access", "Activează accesul direct cu IP"),
        ("Rename", "Redenumește"),
        ("Space", "Spațiu"),
        ("Create desktop shortcut", "Creează comandă rapidă de desktop"),
        ("Change Path", "Schimbă calea"),
        ("Create Folder", "Creează folder"),
        ("Please enter the folder name", "Introdu numele folderului"),
        ("Fix it", "Repară"),
        ("Warning", "Avertisment"),
        ("Login screen using Wayland is not supported", "Ecranele de conectare care folosesc Wayland nu sunt acceptate"),
        ("Reboot required", "Repornire necesară"),
        ("Unsupported display server", "Tipul de server de afișaj nu este acceptat"),
        ("x11 expected", "Este necesar X11"),
        ("Port", "Port"),
        ("Settings", "Setări"),
        ("Username", " Nume utilizator"),
        ("Invalid port", "Port nevalid"),
        ("Closed manually by the peer", "Conexiune închisă manual de dispozitivul pereche"),
        ("Enable remote configuration modification", "Activează modificarea configurației de la distanță"),
        ("Run without install", "Rulează fără a instala"),
        ("Connect via relay", "Conectează-te prin retransmisie"),
        ("Always connect via relay", "Conectează-te mereu prin retransmisie"),
        ("whitelist_tip", "Doar adresele IP autorizate pot accesa acest dispozitiv"),
        ("Login", "Conectează-te"),
        ("Verify", "Verificare"),
        ("Remember me", "Reține-mă"),
        ("Trust this device", "Acest dispozitiv este de încredere"),
        ("Verification code", "Cod de verificare"),
        ("verification_tip", ""),
        ("Logout", "Deconectează-te"),
        ("Tags", "Etichete"),
        ("Search ID", "Caută după ID"),
        ("whitelist_sep", "Poți folosi ca separator virgula, punctul și virgula, spațiul sau linia nouă"),
        ("Add ID", "Adaugă ID"),
        ("Add Tag", "Adaugă etichetă"),
        ("Unselect all tags", "Deselectează toate etichetele"),
        ("Network error", "Eroare de rețea"),
        ("Username missed", "Lipsește numele de utilizator"),
        ("Password missed", "Lipsește parola"),
        ("Wrong credentials", "Nume sau parolă greșită"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Modifică etichetă"),
        ("Forget Password", "Uită parola"),
        ("Favorites", "Favorite"),
        ("Add to Favorites", "Adaugă la Favorite"),
        ("Remove from Favorites", "Șterge din Favorite"),
        ("Empty", "Gol"),
        ("Invalid folder name", "Denumire folder nevalidă"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Nume gazdă"),
        ("Discovered", "Descoperite"),
        ("install_daemon_tip", "Pentru executare la pornirea sistemului, instalează serviciul de sistem."),
        ("Remote ID", "ID dispozitiv la distanță"),
        ("Paste", "Lipește"),
        ("Paste here?", "Lipește aici?"),
        ("Are you sure to close the connection?", "Sigur vrei să închizi conexiunea?"),
        ("Download new version", "Descarcă noua versiune"),
        ("Touch mode", "Mod tactil"),
        ("Mouse mode", "Mod mouse"),
        ("One-Finger Tap", "Apasă cu un deget"),
        ("Left Mouse", "Clic stânga"),
        ("One-Long Tap", "Apasă lung"),
        ("Two-Finger Tap", "Apasă cu două degete"),
        ("Right Mouse", "Clic dreapta"),
        ("One-Finger Move", "Mișcă cu un deget"),
        ("Double Tap & Move", "Apasă dublu și mișcă"),
        ("Mouse Drag", "Tragere mouse"),
        ("Three-Finger vertically", "Trei degete vertical"),
        ("Mouse Wheel", "Rotiță mouse"),
        ("Two-Finger Move", "Mișcă cu două degete"),
        ("Canvas Move", "Mută ecran"),
        ("Pinch to Zoom", "Apropie degetele pentru a mări"),
        ("Canvas Zoom", "Mărire ecran"),
        ("Reset canvas", "Reinițializează ecranul"),
        ("No permission of file transfer", "Nicio permisiune pentru transferul de fișiere"),
        ("Note", "Reține"),
        ("Connection", "Conexiune"),
        ("Share Screen", "Partajează ecran"),
        ("Chat", "Mesaje"),
        ("Total", "Total"),
        ("items", "elemente"),
        ("Selected", "Selectat"),
        ("Screen Capture", "Capturare ecran"),
        ("Input Control", "Control intrări"),
        ("Audio Capture", "Capturare audio"),
        ("File Connection", "Conexiune fișier"),
        ("Screen Connection", "Conexiune ecran"),
        ("Do you accept?", "Accepți?"),
        ("Open System Setting", "Deschide setări sistem"),
        ("How to get Android input permission?", "Cum autorizez dispozitive de intrare pe Android?"),
        ("android_input_permission_tip1", "Pentru ca un dispozitiv la distanță să poată controla un dispozitiv Android folosind mouse-ul sau suportul tactil, trebuie să permiți StarDesk să utilize serviciul „Accesibilitate”."),
        ("android_input_permission_tip2", "Accesează următoarea pagină din Setări, deschide [Aplicații instalate] și pornește serviciul [StarDesk Input]."),
        ("android_new_connection_tip", "Ai primit o nouă solicitare de controlare a dispozitivului actual."),
        ("android_service_will_start_tip", "Activarea setării de capturare a ecranului va porni automat serviciul, permițând altor dispozitive să solicite conectarea la dispozitivul tău."),
        ("android_stop_service_tip", "Închiderea serviciului va închide automat toate conexiunile stabilite."),
        ("android_version_audio_tip", "Versiunea actuală de Android nu suportă captura audio. Fă upgrade la Android 10 sau la o versiune superioară."),
        ("android_start_service_tip", "Apasă [Pornește serviciu] sau DESCHIDE [Capturare ecran] pentru a porni serviciul de partajare a ecranului."),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Cont"),
        ("Overwrite", "Suprascrie"),
        ("This file exists, skip or overwrite this file?", "Fișier deja existent. Omite sau suprascrie?"),
        ("Quit", "Ieși"),
        ("Help", "Ajutor"),
        ("Failed", "Nereușit"),
        ("Succeeded", "Reușit"),
        ("Someone turns on privacy mode, exit", "Cineva activează modul privat, ieși din"),
        ("Unsupported", "Neacceptat"),
        ("Peer denied", "Dispozitiv pereche refuzat"),
        ("Please install plugins", "Instalează pluginuri"),
        ("Peer exit", "Ieșire dispozitiv pereche"),
        ("Failed to turn off", "Dezactivare nereușită"),
        ("Turned off", "Închis"),
        ("Language", "Limbă"),
        ("Keep StarDesk background service", "Rulează serviciul StarDesk în fundal"),
        ("Ignore Battery Optimizations", "Ignoră optimizările de baterie"),
        ("android_open_battery_optimizations_tip", "Pentru dezactivarea acestei funcții, accesează setările aplicației StarDesk, deschide secțiunea [Baterie] și deselectează [Fără restricții]."),
        ("Start on boot", "Pornește la boot"),
        ("Start the screen sharing service on boot, requires special permissions", "Pornește serviciul de partajare a ecranului la boot; necesită permisiuni speciale"),
        ("Connection not allowed", "Conexiune neautoriztă"),
        ("Legacy mode", "Mod legacy"),
        ("Map mode", "Mod hartă"),
        ("Translate mode", "Mod traducere"),
        ("Use permanent password", "Folosește parola permanentă"),
        ("Use both passwords", "Folosește ambele programe"),
        ("Set permanent password", "Setează parola permanentă"),
        ("Enable remote restart", "Activează repornirea la distanță"),
        ("Restart remote device", "Repornește dispozivul la distanță"),
        ("Are you sure you want to restart", "Sigur vrei să repornești dispozitivul?"),
        ("Restarting remote device", "Se repornește dispozitivul la distanță"),
        ("remote_restarting_tip", "Dispozitivul este în curs de repornire. Închide acest mesaj și reconectează-te cu parola permanentă după un timp."),
        ("Copied", "Copiat"),
        ("Exit Fullscreen", "Ieși din modul ecran complet"),
        ("Fullscreen", "Ecran complet"),
        ("Mobile Actions", "Bară de navigare"),
        ("Select Monitor", "Selectează monitor"),
        ("Control Actions", "Acțiuni de control"),
        ("Display Settings", "Setări afișaj"),
        ("Ratio", "Raport"),
        ("Image Quality", "Calitate imagine"),
        ("Scroll Style", "Stil de derulare"),
        ("Show Toolbar", "Arată bară de instrumente"),
        ("Hide Toolbar", "Ascunde bară de instrumente"),
        ("Direct Connection", "Conexiune directă"),
        ("Relay Connection", "Conexiune prin retransmisie"),
        ("Secure Connection", "Conexiune securizată"),
        ("Insecure Connection", "Conexiune nesecurizată"),
        ("Scale original", "Dimensiune originală"),
        ("Scale adaptive", "Scalare automată"),
        ("General", "General"),
        ("Security", "Securitate"),
        ("Theme", "Temă"),
        ("Dark Theme", "Temă întunecată"),
        ("Light Theme", "Temă luminoasă"),
        ("Dark", "Întunecată"),
        ("Light", "Luminoasă"),
        ("Follow System", "Temă sistem"),
        ("Enable hardware codec", "Activează codec hardware"),
        ("Unlock Security Settings", "Deblochează setările de securitate"),
        ("Enable audio", "Activează audio"),
        ("Unlock Network Settings", "Deblochează setările de rețea"),
        ("Server", "Server"),
        ("Direct IP Access", "Acces direct IP"),
        ("Proxy", "Proxy"),
        ("Apply", "Aplică"),
        ("Disconnect all devices?", "Vrei să deconectezi toate dispozitivele?"),
        ("Clear", "Golește"),
        ("Audio Input Device", "Dispozitiv de intrare audio"),
        ("Use IP Whitelisting", "Folosește lista de IP-uri autorizate"),
        ("Network", "Rețea"),
        ("Pin Toolbar", "Fixează bara de instrumente"),
        ("Unpin Toolbar", "Detașează bara de instrumente"),
        ("Recording", "Înregistrare"),
        ("Directory", "Director"),
        ("Automatically record incoming sessions", "Înregistrează automat sesiunile viitoare"),
        ("Change", "Modifică"),
        ("Start session recording", "Începe înregistrarea"),
        ("Stop session recording", "Oprește înregistrarea"),
        ("Enable recording session", "Activează înregistrarea sesiunii"),
        ("Enable LAN discovery", "Activează descoperirea LAN"),
        ("Deny LAN discovery", "Interzice descoperirea LAN"),
        ("Write a message", "Scrie un mesaj"),
        ("Prompt", "Prompt"),
        ("Please wait for confirmation of UAC...", "Așteaptă confirmarea CCU..."),
        ("elevated_foreground_window_tip", "Fereastra actuală a dispozitivului la distanță necesită privilegii sporite pentru a funcționa, astfel că mouse-ul și tastatura nu pot fi folosite. Poți cere utilizatorului la distanță să minimizeze fereastra actuală sau să facă clic pe butonul de sporire a privilegiilor din fereastra de gestionare a conexiunilor. Pentru a evita această problemă, recomandăm instalarea software-ului pe dispozitivul la distanță."),
        ("Disconnected", "Deconectat"),
        ("Other", "Altele"),
        ("Confirm before closing multiple tabs", "Confirmă înainte de a închide mai multe file"),
        ("Keyboard Settings", "Setări tastatură"),
        ("Full Access", "Acces total"),
        ("Screen Share", "Partajare ecran"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland necesită Ubuntu 21.04 sau o versiune superioară."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland necesită o versiune superioară a distribuției Linux. Încearcă desktopul X11 sau schimbă sistemul de operare."),
        ("JumpLink", "Afișează"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Partajează ecranul care urmează să fie partajat (operează din partea dispozitivului pereche)."),
        ("Show StarDesk", "Afișează StarDesk"),
        ("This PC", "Acest PC"),
        ("or", "sau"),
        ("Continue with", "Continuă cu"),
        ("Elevate", "Sporește privilegii"),
        ("Zoom cursor", "Cursor lupă"),
        ("Accept sessions via password", "Acceptă începerea sesiunii folosind parola"),
        ("Accept sessions via click", "Acceptă începerea sesiunii dând clic"),
        ("Accept sessions via both", "Acceptă începerea sesiunii folosind ambele moduri"),
        ("Please wait for the remote side to accept your session request...", "Așteaptă ca solicitarea ta de conectare la distanță să fie acceptată..."),
        ("One-time Password", "Parolă unică"),
        ("Use one-time password", "Folosește parola unică"),
        ("One-time password length", "Lungimea parolei unice"),
        ("Request access to your device", "Solicitare de acces la dispozitivul tău"),
        ("Hide connection management window", "Ascunde fereastra de gestionare a conexiunilor"),
        ("hide_cm_tip", "Permite ascunderea ferestrei de gestionare doar dacă accepți începerea sesiunilor folosind parola permanentă"),
        ("wayland_experiment_tip", "Wayland este acceptat doar într-o formă experimentală. Folosește X11 dacă nu ai nevoie de acces supravegheat."),
        ("Right click to select tabs", "Dă clic dreapta pentru a selecta file"),
        ("Skipped", "Ignorat"),
        ("Add to address book", "Adaugă la agendă"),
        ("Group", "Grup"),
        ("Search", "Caută"),
        ("Closed manually by web console", "Conexiune închisă manual de consola web"),
        ("Local keyboard type", "Tastatură locală"),
        ("Select local keyboard type", "Selectează tastatura locală"),
        ("software_render_tip", "Dacă ai o placă video Nvidia și folosești Linux, iar fereastra cu conexiunea la distanță se închide imediat după conectare, îți sugerăm să instalezi driverul gratuit Nouveau și să folosești randarea de software. Este necesară repornirea."),
        ("Always use software rendering", "Utilizează mereu randarea de software"),
        ("config_input", "Pentru a controla desktopul la distanță folosind tastatura, trebuie să acorzi StarDesk permisiunea Monitorizare intrare"),
        ("config_microphone", "Pentru a desfășura un apel vocal, este nevoie să acorzi StarDesk permisiunea Înregistrare audio."),
        ("request_elevation_tip", "Poți solicita sporirea privilegiilor și dacă este cineva la desktopul la distanță."),
        ("Wait", "În curs..."),
        ("Elevation Error", "Eroare la sporirea privilegiilor"),
        ("Ask the remote user for authentication", "Solicită utilizatorului de la distanță să se autentifice"),
        ("Choose this if the remote account is administrator", "Alege asta dacă contul la distanță este un cont de administrator"),
        ("Transmit the username and password of administrator", "Transmite numele de utilizator și parola administratorului"),
        ("still_click_uac_tip", "Este necesar ca utilizatorul la distanță să confirme în fereastra CCU din StarDesk care rulează."),
        ("Request Elevation", "Solicită sporirea privilegiilor"),
        ("wait_accept_uac_tip", "Așteaptă ca utilizatorul la distanță să accepte dialogul CCU."),
        ("Elevate successfully", "Sporirea privilegiilor realizată cu succes"),
        ("uppercase", "majuscule"),
        ("lowercase", "minuscule"),
        ("digit", "cifre"),
        ("special character", "caractere speciale"),
        ("length>=8", "lungime>=8"),
        ("Weak", "Slabă"),
        ("Medium", "Medie"),
        ("Strong", "Puternică"),
        ("Switch Sides", "Inversează controlul"),
        ("Please confirm if you want to share your desktop?", "Confirmi că dorești să îți partajezi desktopul?"),
        ("Display", "Afișare"),
        ("Default View Style", "Stilul implicit de vizualizare"),
        ("Default Scroll Style", "Stilul implicit de derulare"),
        ("Default Image Quality", "Calitatea implicită a imaginii"),
        ("Default Codec", "Codec implicit"),
        ("Bitrate", "Rată de biți"),
        ("FPS", "CPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Alte opțiuni implicite"),
        ("Voice call", "Apel vocal"),
        ("Text chat", "Conversație text"),
        ("Stop voice call", "Încheie apel vocal"),
        ("relay_hint_tip", "Este posibil să nu te poți conecta direct; poți încerca să te conectezi prin retransmisie. De asemenea, dacă dorești să te conectezi direct prin retransmisie, poți adăuga sufixul „/r” la ID sau să bifezi opțiunea Conectează-te mereu prin retransmisie."),
        ("Reconnect", "Reconectează-te"),
        ("Codec", "Codec"),
        ("Resolution", "Rezoluție"),
        ("No transfers in progress", "Niciun transfer nu este în desfășurare"),
        ("Set one-time password length", "Definește lungimea parolei unice"),
        ("install_cert_tip", "Instalează certificatul StarDesk"),
        ("confirm_install_cert_tip", "Acesta este un certificat de testare StarDesk și este de încredere. Certificatul va fi utilizat pentru a acorda încredere și instala drivere StarDesk atunci când este necesar."),
        ("RDP Settings", "Setări RDP"),
        ("Sort by", "Sortează după"),
        ("New Connection", "Conexiune nouă"),
        ("Restore", "Restaurează"),
        ("Minimize", "Minimizează"),
        ("Maximize", "Maximizează"),
        ("Your Device", "Dispozitivul tău"),
        ("empty_recent_tip", "Hopa! Nu există nicio sesiune recentă.\nPoate ar trebui să plănuiești una chiar acum!"),
        ("empty_favorite_tip", "Încă nu ai niciun dispozitiv pereche favorit?\nHai să-ți găsim pe cineva cu care să te conectezi, iar apoi poți adăuga dispozitivul la Favorite!"),
        ("empty_lan_tip", "Of! S-ar părea că încă nu am descoperit niciun dispozitiv."),
        ("empty_address_book_tip", "Măi să fie! Se pare că deocamdată nu figurează niciun dispozitiv în agenda ta."),
        ("eg: admin", "ex: admin"),
        ("Empty Username", "Nume utilizator nespecificat"),
        ("Empty Password", "Parolă nespecificată"),
        ("Me", "Eu"),
        ("identical_file_tip", "Acest fișier este identic cu cel al dispozitivului pereche."),
        ("show_monitors_tip", "Afișează monitoare în bara de instrumente"),
        ("View Mode", "Mod vizualizare"),
        ("login_linux_tip", "Este necesar să te conectezi la contul de Linux de la distanță pentru a începe o sesiune cu un desktop care folosește X11"),
        ("verify_stardesk_password_tip", "Verifică parola StarDesk"),
        ("remember_account_tip", "Reține contul"),
        ("os_account_desk_tip", "Acest cont este utilizat pentru conectarea la sistemul de operare la distanță și începerea sesiunii cu desktopul în modul fără afișaj."),
        ("OS Account", "Cont OS"),
        ("another_user_login_title_tip", "Un alt utilizator este deja conectat"),
        ("another_user_login_text_tip", "Deconectare"),
        ("xorg_not_found_title_tip", "Xorg nu a fost găsit"),
        ("xorg_not_found_text_tip", "Instalează Xorg"),
        ("no_desktop_title_tip", "Nu este disponibil niciun mediu desktop"),
        ("no_desktop_text_tip", "Instalează mediul desktop GNOME"),
        ("No need to elevate", "Nu sunt necesare permisiuni de administrator"),
        ("System Sound", "Sunet sistem"),
        ("Default", "Implicit"),
        ("New RDP", "RDP nou"),
        ("Fingerprint", "Amprentă digitală"),
        ("Copy Fingerprint", "Copiază amprenta digitală"),
        ("no fingerprints", "Nicio amprentă digitală"),
        ("Select a peer", "Selectează un dispozitiv pereche"),
        ("Select peers", "Selectează dispozitive pereche"),
        ("Plugins", "Pluginuri"),
        ("Uninstall", "Dezinstalează"),
        ("Update", "Actualizează"),
        ("Enable", "Activează"),
        ("Disable", "Dezactivează"),
        ("Options", "Opțiuni"),
        ("resolution_original_tip", "Rezoluție originală"),
        ("resolution_fit_local_tip", "Adaptează la rezoluția locală"),
        ("resolution_custom_tip", "Rezoluție personalizată"),
        ("Collapse toolbar", "Restrânge bara de instrumente"),
        ("Accept and Elevate", "Acceptă și sporește privilegii"),
        ("accept_and_elevate_btn_tooltip", "Acceptă conectarea și sporește privilegiile CCU"),
        ("clipboard_wait_response_timeout_tip", "Procesul a expirat așteptând un răspuns la copiere"),
        ("Incoming connection", "Conexiune de intrare"),
        ("Outgoing connection", "Conexiune de ieșire"),
        ("Exit", "Ieși"),
        ("Open", "Deschide"),
        ("logout_tip", "Sigur vrei să te deconectezi?"),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_StarDesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("switch_display_elevated_connections_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("capture_display_elevated_connections_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
    ].iter().cloned().collect();
}
