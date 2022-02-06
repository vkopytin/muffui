# MuffUI
Experimental UI framework for windows GUI API (win32). To build windows applications with the developer experience similar to recent web technologies. Implemented with Rust programming language. In the form of declarative layout syntax.

This is the proof of concept project. Declarative UI. For the Rust language. To check how would be developing experience if use UI implemented with the model view update/mvvm. Global variables oriented development.

To build project exec:
```
$cargo build
```

To run project exec:
```
$cargo run
```

This is how it works (TODO example):

![ToDO example](/media/how-it-works.gif?raw=true "ToDO example on MuffUI")

The  solution has a small set of features.
Implemented controls:
* Window
* Group box
* Panel
* Label
* TextBox (Edit box)
* Radio box
* Check box
* Combobox (dropdown)

Every control has the same set of settings. Such as:
* PosX, PosY, Width, Height, Title, Control Index.

Implemented simple resize algorithm. A bit buggy. Works slow. But it works.

Utilised design patterns:
* ELM/Model View Update (MVU)
* Observable (Event Hub)
* Model View ViewModel (MVVM)

# Conclusion:

Declarative UI/declarative syntax is the future of UI development. It helps to build a UI that is easy to maintain, understand and extend.

It could serve as a basis for multi OS supported UI. Such UI would be easy to backport to another platform/operating system.
