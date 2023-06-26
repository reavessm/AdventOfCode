PROGRAM day_one_part_one
IMPLICIT NONE

  INTEGER :: s, m1, m2, m3 
  INTEGER :: stat
  INTEGER :: line

  OPEN (1, file = 'calories.txt', status = 'old', action = 'read', iostat=stat)
  IF (stat /= 0) STOP 'ERROR'

  s = 0
  m1 = 0
  m2 = 0
  m3 = 0

  DO
    READ(1, '(I6)', iostat=stat) line
    IF (stat /= 0 ) EXIT

    IF (line == 0) THEN
      IF (s >= m1) THEN
        m3 = m2
        m2 = m1
        m1 = s
      ELSE IF (s >= m2) THEN
        m3 = m2
        m2 = s
      ELSE IF (s >= m3) THEN
        m3 = s
      END IF
      s = 0
    ELSE
      s = s + line
    END IF
  END DO

  CLOSE(1)

  PRINT '(I6, A, I6, A, I6, A, I6)', m1, " ",  m2 , " ", m3, " = ", m1 + m2 + m3
END PROGRAM day_one_part_one
