-- Моя первая программа на Metal

say "Добро пожаловать в Metal!"

name = "Daniil"
say "Привет, " + name + "!"

sum = 0
for i in 1..10
  sum = sum + i
end
say "Сумма 1..10 = " + sum

fn factorial(n)
  if n <= 1
    return 1
  end
  n * factorial(n - 1)
end

say "5! = " + factorial(5)
say "10! = " + factorial(10)

fn is_even(n)
  if n % 2 == 0
    return true
  end
  return false
end

for i in 1..10
  if is_even(i)
    say str(i) + " — чётное"
  else
    say str(i) + " — нечётное"
  end
end

say "Готово!"