\version "2.20.0"

\header {
  title = "C major scale"
  composer = ""
  tagline = \markup {
    Engraved at
    \simple #(strftime "%Y-%m-%d" (localtime (current-time)))
    with \with-url #"http://lilypond.org/"
    \line { LilyPond \simple #(lilypond-version) (http://lilypond.org/) }
  }
}

rhMusic = \relative c'' {
  c4 d e f | g a b c \bar "|."
}

\score {
  \new PianoStaff <<
    \new Staff = "RH"  <<
      \key c \major
      \rhMusic
    >>
  >>
  \midi {}
}
