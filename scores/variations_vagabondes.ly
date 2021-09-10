\version "2.20.0"

\header {
  title = "Variations Vagabondes Var. 1 Excerpt"
  composer = "Berto Vaudrawn"
  tagline = \markup {
    Engraved at
    \simple #(strftime "%Y-%m-%d" (localtime (current-time)))
    with \with-url #"http://lilypond.org/"
    \line { LilyPond \simple #(lilypond-version) (http://lilypond.org/) }
  }
}

rhMusic = \relative c'' {
  \time 6/8
  \tempo "Andante" 4. = 72
  <<
    {
      cis4.~ \override Stem.direction = #UP cis8 dis8. e16 |
      \tieUp cis4.~ cis8 dis8. e16 |
      cis4.~ cis8 dis8. cis16 |
      gis2.
    }
    \new Voice {
      \voiceTwo
      r8 gis16 cis, e gis, gis'4 e8 |
      r8 a16 cis, e a, g'4 e8 |
      r8 ais16 cis, dis fisis, fisis'4 dis8 |
      r8 fis16 gis, cis dis bis8 fis'16 gis, dis' bis
    }
  >>
  \bar "|."
}

lhMusic = \relative c {
  \set Staff.pedalSustainStyle = #'mixed
  cis,4.\sustainOn r8. <cis cis'>8\sustainOff\sustainOn r16\sustainOff |
  <a a'>8\sustainOn r4 r8. <a a'>8\sustainOff\sustainOn r16\sustainOff |
  <dis, dis'>8\sustainOn r4 r8. <dis dis'>8\sustainOff\sustainOn r16\sustainOff |
  <gis gis'>8\sustainOn r4 gis'8\sustainOff\sustainOn r4\sustainOff \bar "|."
}

\score {
  \new PianoStaff <<
    \new Staff = "RH"  <<
      \key cis \minor
      \rhMusic
    >>
    \new Staff = "LH" <<
      \key cis \minor
      \clef "bass"
      \lhMusic
    >>
  >>
  \midi {}
}