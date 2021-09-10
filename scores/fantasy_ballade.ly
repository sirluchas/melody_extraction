\version "2.20.0"

\header {
  title = "Fantasy-Ballade"
  composer = "David Dinh and Mr Peabody"
  tagline = \markup {
    Engraved at
    \simple #(strftime "%Y-%m-%d" (localtime (current-time)))
    with \with-url #"http://lilypond.org/"
    \line { LilyPond \simple #(lilypond-version) (http://lilypond.org/) }
  }
}

rhMusic = \relative c'' {
  \time 4/4
  \tempo 4 = 144
  <a a'>4 <d d'>( <bes bes'>4. <a a'>8) |
  <aes aes'>8( <ees' ees'> <c c'> <aes aes'>) <g g'>4( <fis fis'>4) |
  <g g'>( <ees ees'>) <aes aes'>( <c c'>) |
  <cis cis'>4.( <bes bes'>8) <a a'>4( <d d'>) |
  <d d'>( <a' a'>) <fis fis'>( <d d'>) |
  <c c'>8( <ees ees'> <d d'> <c c'>) <bes bes'>4( <g g'>4) |
  <a a'> <d d'>~( <d d'>8 <a a'> <f f'> <d d'>) |
  <e e'>4( <a a'>) <d, d'>2 |
  a'4( <d, d'> <bes bes'>4. a'8) |
  <<
    \relative c'' {
      aes8( ees' c aes g4 fis) |
      g( ees <aes, aes'> <c c'>) |
      <cis cis'>4.( <bes bes'>8 <a a'>4 <d d'>) |
      <d d'>( <a' a'> <fis fis'> <d d'>) |
      <c c'>8( <ees ees'> <d d'> <c c'> <bes bes'>4 <g g'>) |
    }
    \\
    \relative c' {
      s8 ees4. c2 |
      bes ees |
      s8 a( gis g fis2) |
      d' bes |
      a g |
    }
  >>
  <a, a'>4( <d d'>4.) a'8( f d) |
  e4( <a, a'> d2) |
}

lhMusic = \relative c {
  fis'16 d a d a fis d d, g d' g, d' bes' d, g bes |
  c aes ees aes ees c aes aes, d d' d a' c fis, d a' |
  bes g ees g ees bes ees, bes' aes ees' aes c aes ees c aes |
  a e' g a cis a g e d, a' d fis a fis a d |
  fis d a d a fis d d, d' d, d' d a' d, fis a |
  d a fis d a' d, a d, g, g' g d' bes' g d bes' |
  a f d a a, a' a d f d f a d a f d |
  a e' a cis e cis a e d, d' fis c' d d, d, d' |
  a' d,, a' d d c' fis, d g d g, g g, g' d' g |
  aes ees aes, aes aes, aes' des ees g ees d, ees' fis d a d, |
  ees bes' f' fis g ees bes g ees ees' f g aes g ees aes, |
  ees cis' g' a e' a, g cis, d, a' d fis c' a fis d |
  d, d' fis g a d, c' cis d c bes a g fis d a |
  d, a' d a fis' d a d, bes' g' bes c d c bes g |
  a, e' f gis a cis ees e f d bes a g e d bes |
  a f' e bes a g f e d a' d e f fis g gis |
}

\score {
  \new PianoStaff <<
    \new Staff = "RH"  <<
      \key g \minor
      \rhMusic
    >>
    \new Staff = "LH" <<
      \key g \minor
      \clef "bass"
      \lhMusic
    >>
  >>
  \midi{}
}