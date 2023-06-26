program day_one_part_one
implicit none

  INTEGER, ALLOCATABLE :: calories(:)
  INTEGER :: s, m, elf_num
  INTEGER :: stat
  INTEGER :: line

  OPEN (1, file = 'calories.txt', status = 'old', action = 'read', iostat=stat)
  IF (stat /= 0) stop 'ERROR'

  s = 0
  m = 0
  elf_num = 1

  DO
    READ(1, '(I4)', iostat=stat) line
    if (stat /= 0 ) exit
    if (line == 0) then
      m = MAX(s, m)
      s = 0
      elf_num = elf_num + 1
    else
      s = s + line
    end if
  END DO

  CLOSE(1)

  PRINT '(A, I5, A, I5, A)', "The ", elf_num, " elf had ", m, " calories."
end program day_one_part_one
