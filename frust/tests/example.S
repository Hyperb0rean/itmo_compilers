addi x2, x0, 32
lui x5, 0
addi x5, x0, 48
sw x2, 32, x5
lw x5, x2, 32
addi x6, x5, 0
lui x5, 0
addi x5, x0, 53
slt x5, x6, x5
beq x5, x0, 17
lw x5, x2, 32
addi x6, x5, 0
lui x5, 0
addi x5, x0, 50
sge x5, x6, x5
beq x5, x0, 4
lui x5, 0
addi x5, x0, 56
sw x2, 32, x5
jal x0, 6
lw x5, x2, 32
addi x6, x5, 0
lui x5, 0
addi x5, x0, 1
add x5, x6, x5
sw x2, 32, x5
jal x0, -23
