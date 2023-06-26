program day_one_part_one
implicit none

  INTEGER, ALLOCATABLE :: calories(:)
  INTEGER :: s, m1, m2, m3 
  INTEGER :: stat
  INTEGER :: line

  OPEN (1, file = 'calories.txt', status = 'old', action = 'read', iostat=stat)
  IF (stat /= 0) stop 'ERROR'

  s = 0
  m1 = 0
  m2 = 0
  m3 = 0

  DO
    READ(1, '(I6)', iostat=stat) line
    if (stat /= 0 ) exit

    if (line == 0) then
      if (s >= m1) then
        m3 = m2
        m2 = m1
        m1 = s
      else if (s >= m2) then
        m3 = m2
        m2 = s
      else if (s >= m3) then
        m3 = s
      end if
      s = 0
    else
      s = s + line
    end if
  END DO

  CLOSE(1)

  PRINT '(I6, A, I6, A, I6, A, I6)', m1, " ",  m2 , " ", m3, " = ", m1 + m2 + m3
end program day_one_part_one
