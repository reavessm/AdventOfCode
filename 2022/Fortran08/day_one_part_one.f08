PROGRAM day_one_part_one
IMPLICIT NONE

  INTEGER :: s, m, elf_num
  INTEGER :: stat
  INTEGER :: line

  OPEN (1, file = 'calories.txt', status = 'old', action = 'read', iostat=stat)
  IF (stat /= 0) STOP 'ERROR'

  s = 0
  m = 0
  elf_num = 1

  DO
    READ(1, '(I4)', iostat=stat) line
    IF (stat /= 0 ) EXIT
    IF (line == 0) THEN
      m = MAX(s, m)
      s = 0
      elf_num = elf_num + 1
    ELSE
      s = s + line
    END IF
  END DO

  CLOSE(1)

  PRINT '(A, I5, A, I5, A)', "The ", elf_num, " elf had ", m, " calories."
END PROGRAM day_one_part_one
