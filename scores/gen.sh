for MIDI_NAME in *.midi; do
  BASE_NAME=$(echo "$MIDI_NAME" | cut -d "." -f 1)
  MP3_NAME="$BASE_NAME.mp3"
  OUT_PATH="../audio/$MP3_NAME"
  TMP_OUT="../audio/$(uuid).mp3"

  if [[ ! -f "$OUT_PATH" ]]; then
    timidity -Ow -o - "$MIDI_NAME" | ffmpeg -i - -acodec libmp3lame -ac 1 -ar 44100 "$OUT_PATH"

    duration=$(mediainfo "$OUT_PATH" --Inform="Audio;%Duration%")
    ffmpeg -to $(($duration / 1000 - 1)) -i "$OUT_PATH" -c copy "$TMP_OUT"
    mv "$TMP_OUT" "$OUT_PATH"
  fi
done
