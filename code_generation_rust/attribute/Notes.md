#custom_attribute
Ein neues Attribut ist ähnlich der Erstellung eines derives. Man erstellt
ein crate des types proc-macro und definiert in diesem die Attribute.
Eine solche Funktion nimmt 2 TokenStreams entgegen, die Parameter in Klammern
und das Item, welches mit dem Attribut versehen wurde.

Um jetzt Code zu generieren, idealerweise mit Parametern, müssen die Attribute
geparsed und dann der neue Code ertsellt werden. Für ersteres kann vermutlich
syn benutzt werden.