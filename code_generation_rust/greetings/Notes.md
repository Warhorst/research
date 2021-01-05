#custom_derive
Ermöglicht das schreiben von eigenen derive-Aufrufen für Structs. Grundsätzlich
besteht eine solche Library aus 2 Modulen: ein Modul mit dem Trait und ein Modul
mit der Implementation des Derive. Letzteres ist ein Submodul des ersten und 
ist vom Typ proc-macro (siehe toml). In einer Library dieses Typs können nur 
proc_macros bereit gestellt werden.

Die Libraries syn und quote werden üblicherweise für die Erstellung solcher Macros
genutzt. Die Methode selbst ist aufgeteilt in die Erstellung des AST aus
dem TokenStream mit syn un der Erstellung eines neuen TokenStreams (der generierte Code)
mit quote.