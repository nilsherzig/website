# Hardware 2 - Laborprotokoll 1

- Hannes Burmeister 20413
- Nils Herzig 19473

# 1. Ehrenwörtliche Erklärung

"Hiermit versichere wir, Nils Herzig und Hannes Burmeister, ehrenwörtlich, dass wir das vorliegende Laborprotokoll mit dem Titel: "Laborprotokoll 1" selbstständig und ohne fremde Hilfe verfasst und keine anderen als die angegebenen Hilfsmittel benutzt haben. Die Stellen der Arbeit, die dem Wortlaut oder dem Sinn nach anderen Werken entnommen wurden, sind in jedem Fall unter Angabe der Quelle kenntlich gemacht. Die Arbeit ist noch nicht veröffentlicht oder in anderer Form als Prüfungsleistung vorgelegt worden."

Stralsund, 06.05.2023, Nils Herzig

Stralsund, 06.05.2023, Hannes Burmeister

# 2. Exercises

## 2.1. Configuration of Digital I/O Port

### 2.1.1. Explaint he meaing of ...

- `P1SEL &= ~BIT7` das 7. Bit in P1SEL wird auf 0 gesetzt.
- `P1SEL |= BIT7` das 7. Bit in P1SEL wird auf 1 gesetzt.
- `P1SEL ^= BIT7` das 7. Bit in P1SEL wird invertiert.

### 2.1.2. In Fig. 4 ...

`P1REN` ist register, in welchem der Pullup / pulldown resistor status von dem jeweiligen I/O Pin, dessen Nummer mit dem jeweiligen Bit Stelle übereinstimmt, gesetzt werden kann.

- `0` -> disabled
- `1` -> enabled

In diesem Fall schalten wir also den Pullup / Pulldown Resistor von dem 3. I/O Pin an, in unserem Fall ist dies `P1.3`.

## 2.2. Setting the CPU Clock

### 2.2.1. Write a C program that sets the CPU frequency to 12MHz

```c
if (CALBC1_12MHZ != 0xFF)
{
	DCOCTL = 0; // Select lowest DCOx and MODx
	BCSCTL1 = CALBC1_12MHZ; // Set range
	DCOCTL = CALDCO_12MHZ; // Set DCO step + modulation
}
```

### 2.2.2. Write a C program taht sets the CPU frequenzy to 6MHz (consult the user guide)

<!-- TODO
Irgendwas von wegen das ding via externem quarz mit 32,768kHz einstellen.
https://www.mikrocontroller.net/topic/69302 -->

## 2.3. Usage of Typedef

### 2.3.1. What are and how to use ...

```c
typedef unsigned char Bool_t;
typedef unsigned short UInt16_t;
typedef signed short Int16_t;
```

Wir definieren 3 neue Datentypen `Bool_t`, `UInt16_t` und `Int16_t`. Diese können wie "normale" Datentypen im weiteren C Code genutzt werden.

## 2.4. Usage of `#defines`

### 2.4.1. Explain he reason for the ...

- `ifndef` verhindert das erneute deklarieren von Werten via define.
- `define` ist ein makro, mit welchem C-Code einer Zeichenkette zu geordnet werden kann. Der compiler kann dann diese Zeichenkette im folgenden Code mit dem zugeordneten C-Code ersetzen.
- `endif` ende des `ifndef` branches

### 2.4.2. How is a ...

`#define COUNT_DELAY_MAX 0x0fff`
Der compiler setzt alle matches von `COUNT_DELAY_MAX` im nachfolgenden C-Code mit `4095`.

## 2.5. Declaration and Definiton

### 2.5.1. Explain the meaing of ...

```c
define define_example 5;
int main()
{
	int declaration_example = 10;
	int result = declaration_example + define_example;
	return 0;
}
```

resultiert in:

```assembly
main:
	push rbp
	mov rbp, rsp
	mov DWORD PTR [rbp-4], 10
	mov eax, DWORD PTR [rbp-4]
	add eax, 5
	mov DWORD PTR [rbp-8], eax
	mov eax, 0
	pop rbp
	ret
```

## 2.6. Setup and Build of a typical Project

### 2.6.1. Discuss in detail

Aus C-Dateien eine binary zu produzieren, erfordert mehrere Schritte:

1. Proprocessing:
   1. Alle preprocessor Anweisungen im Code ausführen, bspw `#include`, `#define` oder `#if`
2. Compilation:
   1. Den output des preprocessors in Maschinencode übersetzen. Je nach Einstellungen können hier auch optimierungen oder symbole umgesetzt werden.
3. Linking:
   1. Eine oder mehrere Dateien (output des compilers) in eine einzige ausführbare binary zusammenfassen..
   2. runtime linking von shared libaries
4. Locating:
   1. Nur in einigen Szenarien nötig
   2. lädt den binary code in memory und inizialisiert nötige datenstrukturen

### 2.6.2. Normally, a software projhect consists of several source files

#### 2.6.2.1. What does a source file contain?

Die implementierung von einer oder mehrer funktionen (bei interpretierten Sprachen teilweise auch nur einzelne Anweisungen) und deren Variablen, Konstanten und Makros. Diese Dateien werden zu binary-code compiled.

#### 2.6.2.2. What exactly means `#include`?

In der Regel bindet man damit header Dateien in ein oder mehrere source Dateien ein.

#### 2.6.2.3. Explain the content and role of a header file

Header files `.h` können via `#include` in mehrer source Files eingebunden werden. Dies ermöglicht es Funktionen, Vars, etc. zwischen mehreren Files zu teilen und verhindert das redundante definieren von häufig genutzten Codeblöcken.

Häufig werden header files für

- function prototypen
- struct definitionen
- makros

gentutz.

# 3. Experiment

## 3.1. 1.3.1 Dice_Blinky

### 3.1.1. Analyse / discuss the program in detail by visual inspection.

- Als erstes wird die LED auf die Standardeinstellungen zurückgesetzt und die Eingabe vom Knopf umgekehrt. Wird also der Knopf gedrückt, wird ein logisches LOW gesendet. Als nächstes wird in einer while schleife dauerhaft überprüft, ob der Knopf gedrückt ist. Sollte der Knopf nicht gedrückt sein, so werden die LEDs eingeschaltet und das Programm loopt sich. Sollte doch der Knopf gedrückt sein, so wird die Variable uCount auf den Anfangswert `COUNT_START` gesetzt und danach durch jede Iteration um einen verringert. Danach wird der Status der LEDs überprüft. Sollten diese an sein, werden sie ausgeschaltet, und andersrum.

### 3.1.2. What does the program do from the users point of view?

- Wird der Knopf nicht gedrückt, werden beide LEDs aktiviert. Wird der Knopf gedrückt, werden beide LEDs aktiviert, danach werden beide an und danach wieder aus geschaltet.

### 3.1.3. How is the functionality realized by the program in detail? Debug the program. Use breakpoints and single-stepping to support your arguing.

- Das Programm hat nicht die Möglichkeit eine LED dauerhaft zu aktivieren. Diese Limitierung wird mittels einer while Schleife umgangen. Somit ist der Code bzw. der Prozessor schneller mit der Durchführung dieser Schleife, als das Licht auf dem Dice deaktiviert wird, und somit ausgeschaltet wird.

### 3.1.4. How was it achived to have the LEDs shine less brightly when the pushbutton is pressed?

- Die Helligkeit lässt sich durch die Iteration von `MASK_DIM_OFF` und `MASK_DIM_PER` erreichen. Durch die Anzahl der Iteration lässt sich die Helligkeit festlegen und manipulieren. Dieser Abfall an lligkeit der aufkommt, wenn der Knopf gedrückt wird, hängt mit der Iteration durch die while Schleife zusammen.

### 3.1.5. Expand the program so that when the pushbutton is not pressed, the LEDs flash according to those when the button is pressed. However, the brightness of the LEDs shall be maximum in this case.

```c
    while(1)
    {
        uCount = COUNT_START;
        P1DIR |= MASK_LEDS;
        if(P1IN & MASK_BUTTON)
        {
            P1OUT ^= MASK_LEDS;
            while(uCount > 0)
            {
                uCount--;
            }
        }
    }
```

### 3.1.6. Calculate the execution time of your delay loop by inspecting the OPCODES in the debugger. Compare your calculation to the experiment.

- Insgesamt beträgt eine Zyklusdauer `3µs` und die insgesamt tatsächliche Leuchtdauer bei `0.38s`

### 3.1.7. Calculate the correct value for COUNT_START, based on the required CPU cycles of the loop, to get a blink period of one second.

- Hierfür nehmen wir den Wert der der Variable `COUNT_START` bereits gegeben wurde. Dieser lautet `0x20000ul`. Betrachtet man diesen Wert im Detail, erkennt man, das es sich hier nicht direkt um einen hexadezimalen Wert handelt, sondern vielmehr um einen unsigned long. Dieser hat die Größe `20.000`. Daher wissen wir nun, dass mit diesem Wert alle `0,88` Sekunden die LEDs blinken. Die 0,88s sind durch die vorhergegangenen Berechnungen ergründbar und ist ebenfalls mit den Zeitmessungen und dem Debugger / Disassembler vergleichbar. Somit können wir nun den Dreisatz anwenden: `20.000 / 0,88` und bekommen das Ergebnis `25.000`. Somit muss die Variable `COUNT_START` von `0x20000ul` auf `0x25000ul` erhöht werden, damit die LEDs jede Sekunde blinken.

## 3.2. Dice_0

### 3.2.1. Describe the interaction of the involved source and header files.

- Die Headerdatei `msp430.h` bietet in diesem Fall nur eine Art Schnittstelle zwischen dem C Programm und dem MSP430. Somit lassen sich beispielsweise Taktfrequenzen der CPU einstellen und der Eigenschaften der Pins im Programm verändern. Im Gegensatz dazu bietet die Headerdatei `Dice_Display.h` eine Schnittstelle für das zuvor angeschlossene Daughterboard 'Dice'. Es bietet grundlegende Zahlenanzeigen für dieses Board und zeigt einige essentielle Funktionen für das erfolgreiche Funktionieren des 'Dice' auf.

### 3.2.2. Describe the build process for the given project.

- Als erstes werden die beiden Header-Dateien geladen, worin sich wichtige vordefinierte Variablen für den Gebrauch vom 'Launchpad' und 'Dice' befinden. Somit werden als erstes alle `#define` Anweisungen und danach alle `#include` Anweisungen compiled. Danach geht es dann zum Hauptprogramm `Dice_0.c`. Der Linker fügt nachher bzw. linkt alle daraus entstandenen object files zusammen zu einer einzigen executable.

### 3.2.3. Describe the array c0to6 regarding its usage, type, and content.

- Der `char` Array `c0to6` dient grundsätzlich für die Anzeige von Zahlen auf dem 'Dice'. Hierzu wird ihm die vordefinierte Variable `COUNT_LOOP_MAX + 1` als Größe gesetzt. Das dient dazu, dass die Variable `COUNT_LOOP_MAX` nur 6 `chars` beinhalten kann. Das wären dann die Zahlen 1 - 6. Will man allerdings noch eine 0 extra auf dem Display anzeigen lassen, so muss der dafür zuständige Array `c0to6` einen extra Wert beinhalten können. Mit dem entprechenden Aufruf kann man so also verschieden Zahlen auf dem 'Dice' darstellen.

### 3.2.4. Describe the procedures Dice_InitDisplay and Dice_SetDisplay.

- Die Methode `Dice_InitDisplay` dient dafür, als erstes einen initialen Wert für das Display zu setzen. Im Gegensatz dazu wird `Dice_SetDisplay` für das setzen eines bestimmen Wertes verwendet. So kann man beispielsweise `Dice_SetDisplay(2)` aufrufen, um die Zahl 2 auf dem 'Dice' zu sehen.

### 3.2.5. Analyze what happens if Dice_SetDisplay(ucCountLoop + 0x20); will be used?

- In diesem Fall würde der ucCountLoop erst nach 32 'normalen' Schritten anfangen. Somit würden die ersten 32 Werte übersprungen werden.

### 3.2.6. Why is the line WDTCTL = WDTPW + WDTHOLD; more or less always required during the lab sessions?

- Diese Zeile stoppt den Watchdotimer, welcher nach einer bestimmten Zeit nach dem laufen des Programms dieses anhält, um eventuelle Abstürze oder Freezes vorzubeugen.

### 3.2.7. Adjust COUNT_DELAY_MAX such that each number is displayed for about one second.

- Für diese Aufgabe können wir den bereits vorher ausgerechneten Wert von `COUNT_START` benutzen, der bereits aussagt, wie groß ein solcher Wert sein muss, um eine Iteration mittels einer Inkrementierung oder Dekrementierung genau um eine Sekunde zu verzögern. Der zuvor errechnete Wert von `COUNT_START` belief sich auf `25.000` Iterationen für eine Sekunde Delay. Somit müssen wir die `20.000` nur in eine Dezimalzahl umrechnen. Dabei kommen wir auf `0x61a8`.

### 3.2.8. Adjust the program such that the display never gets completely off

```c
while(1)
  {
    //! - Counter for the display (index)
    unsigned char ucCountLoop;

    //! - Loop over output array
    for (ucCountLoop=1; ucCountLoop <= COUNT_LOOP_MAX; ucCountLoop++)
    {
      //! - -- Set Display
      Dice_SetDisplay(ucCountLoop);

      //! - -- Delay for a while
      for (usCountDelay=0; usCountDelay < COUNT_DELAY_MAX; usCountDelay++);
    }
  }
```

Mit der Inkrementierung von der Variable `ucCountLoop` fängt das Display nun bei der Zahl 1 an und fängt bei 6 an sich zu wiederholen. Somit kommt es nicht zu der Ausgabe einer 0 und kommt auch nicht zu einem deaktivierten Display.

### 3.2.9. Disassembly view: What does the line below mean in detail? `00C020 40B2 5A80 0120 mov.w #0x5A80,&WDTCTL`

- Diese Assembly Anweisung bewirkt, dass in das Register `WDTCTL` der Wert `0x5A80` geladen wird. `00C020` ist die "Adresse" auf welcher der Befehl gespeichert bzw. geschrieben ist. `40B2` stellt dabei den Opcode da, und zeigt den Assemble, was er genau zu tun hat. `5A80` ist dabei der zu bewegende Wert. `0120` ist dabei der Programmcounter.

### 3.2.10. Where (physically, in detail) are usCountDelay, ucCountLoop, and c0to6 located? Use the debugger to inspect/view the corresponding memory regions.

- Während `ucCountLoop` im Register R10 zu finden ist, befindet sich `usCountDeplay` im RAM und `c0to6` wird bereits vor Programmstart in den Registers initialisiert.

### 3.2.11. What is the starting address of the Program?

- Das Programm startet an der Adresse `00C008`

### 3.2.12. Use View::Memory::Memory1 to display the memory of the MSP430. In the drop-down list labeled GoTo enter main. Discuss the displayed value with respect of the prior questions.

- Hier zu erkennen ist das tatsächliche Wert, der im RAM bzw. im Memory gespeichert ist. Darauf zu erkennen sind verschiedene Hex Werte, die teilweise Pointer sind.

### 3.2.13. Inspect the label \_\_program_start by using the memory browser and compare the highlighted value with the same label in the disassembly view (View::Disassembly)

- Die beiden hervorgehobenen Werte sind die gleichen, was darauf schließen lässt, das das Programm bzw. der Programmstart erflogreich in den RAM geladen werden konnte. Ebenfalls kann man durch die genau Analyse des Memory Browsers ebenfalls eine Art Disassembly vornehmen, das sich die Werte hier wie bereits erwähnt überschreiben.

### 3.2.14. Inspect the content of address 0xFFFF with the memory browser. What is the relation to the value from the prior question?

- Der Wert am Ende vom Memory ist der gleiche Wert wie der aktuelle Programmcounter. Somit lässt sich eine direkte Beziehung zwischen diesen beiden Werten aufbauen. Der Programmcounter hat an der Stelle vom Programmstart den gleichen Wert, den man auch am Ende vom Memory finden kann.

### 3.2.15. Inspect the memory for c0to6. Discuss the values and mnemonics shown in the disassembly view for the same label.

- Dieses Abbild vom Memory zeigt welche Werte als erstes für c0to6 erstellt wurden.

### 3.2.16. Discuss in detail what means static const unsigned char c0to6[7] = {…} from Dice_Display.c. Where/when are the involved calculations executed?

- Diese Operationen werden beim aufrufen der Headerdateien ausgeführt. Somit wird auch dieser Wert im Programm noch vor dem richtigen Programmstart definiert und initialisiert. Gleiche Berechnungen werden beim Aufruf entsprechender Methoden ausgefährt.

### 3.2.17. Discuss the generated file Dice_0.html.

- Diese Datei zeigt viele wichtige Offsets und Addressen von wichtigen Punkten aus dem Programm. So lässt sich beispielsweise der Programmstart genaustens erkennen oder auch die Adressen vieler Variablen.

### 3.2.18. Discuss the generated list files Dice_0.lst and Dice_Display.lst.

- Die einzelnen .lst Dateien zeigen noch weitere Einzelheiten direkt im Code auf. Somit kann man auch im Quellcode direkt sagen, an welcher Speicheradresse oder in welchen Register sich bestimmte Variablen schon finden lassen.

### 3.2.19. Discuss the generated preprocessed files Dice_0.i and Dice_Display.i located in the debug folder for the project.

- Die verschiedenen Dateien die auf .i Enden sind mit C und dem Precompilen in Verbindung zu bringen. Somit lassen sich in der Datei Methoden und andere Datenstrukturen definieren, welche nicht vom Precompiler erfasst werden sollen. Sie werden somit praktisch erst bei Runtime compiled.

## 3.3. Dice_1 - Pushbutton

### 3.3.1. Discuss bouncing with respect to pushbuttons.

- Da die Metallplatten, aus dem der Button gemacht ist, zuteils elastisch ist, kommt es zu Mehrfachinputs

### 3.3.2. Enhance Dice_1_lab.c to fulfill the requirements from above.

```c
while ( !Dice_IsButtonPressed() );
  ucCountLoop++;
  if (ucCountLoop > COUNT_LOOP_MAX) {
    ucCountLoop = 1;
  }
  Dice_SetDisplay(ucCountLoop);
```

### 3.3.3. Analyze and discuss the Dice_1.html.

- Wie bereits erwähnt zeigt diese HTML Datei ebenfalls verschiedene Speicheradressen oder ähnliche Werte auf, die für das Programm von Verlangen sind.

## Dice_2 - Pushbutton and random number generator

### Enhance the given program

```c
#include "msp430.h"
#include "Dice_Display.h"
#include "Dice_Button.h"
#include "rand.h"
#define COUNT_DELAY_MAX 0x0fff
unsigned int ucRandom;
int main(void) {
    WDTCTL = WDTPW + WDTHOLD;
    Dice_InitDisplay();
    Dice_InitButton();
    ucRandom = rand();
    while(1) {
        while(!Dice_IsButtonPressed());
        ucRandom = prand(&ucRandom);
        Dice_SetDisplay(ucRandom % 6 + 1);
        while(Dice_IsButtonPressed());
    }
}
```
