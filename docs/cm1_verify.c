// CM-1 Verifier — Binary Split Geometry
// Authority: SMCA/1
// Status: Frozen v1

typedef enum {
  S_IDLE,
  S_HEADER,
  S_ACTIVE,
  S_SUBSCOPE,
  S_CLOSING,
  S_COLLAPSE,
  S_ILLEGAL
} cm1_state;

cm1_state verify_cm1(const uint8_t* stream, size_t len) {
  cm1_state state = S_IDLE;
  int scope_depth = 0;
  int seen_eot = 0;

  for (size_t i = 0; i < len; i++) {
    uint8_t c = stream[i];

    switch (state) {

      case S_IDLE:
        if (c == 0x01) state = S_HEADER;     // SOH
        else return S_ILLEGAL;
        break;

      case S_HEADER:
        if (c == 0x02) state = S_ACTIVE;     // STX
        else return S_ILLEGAL;
        break;

      case S_ACTIVE:
        if (c == 0x0E) {                     // SO
          scope_depth++;
          state = S_SUBSCOPE;
        }
        else if (c == 0x03) {                // ETX
          if (scope_depth != 0) return S_ILLEGAL;
          state = S_CLOSING;
        }
        else return S_ILLEGAL;
        break;

      case S_SUBSCOPE:
        if (c == 0x1E) {                     // RS
          /* allowed: internal subdivision */
        }
        else if (c == 0x0F) {                // SI
          scope_depth--;
          if (scope_depth < 0) return S_ILLEGAL;
          state = S_ACTIVE;
        }
        else return S_ILLEGAL;
        break;

      case S_CLOSING:
        if (c == 0x04) {                     // EOT
          if (seen_eot) return S_ILLEGAL;
          seen_eot = 1;
          state = S_COLLAPSE;
        }
        else return S_ILLEGAL;
        break;

      case S_COLLAPSE:
        return S_ILLEGAL; // nothing allowed after collapse
    }
  }

  return (state == S_COLLAPSE) ? S_COLLAPSE : S_ILLEGAL;
}
