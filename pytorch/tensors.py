from __future__ import print_function
import torch

# somehow contains values
x = torch.empty(5, 3)
print(x)

x = torch.rand(5, 3)
print(x)

x = torch.zeros(5, 3, dtype=torch.long)
print(x)

x = torch.tensor([5.5, 3])
print(x)

x = x.new_ones(5, 3, dtype=torch.double)
print(x)

x = torch.randn_like(x, dtype=torch.float)
print(x)

print(x.size())

# add, get result
y = torch.rand(5, 3)
print(x + y)

# add, get result syntax 2
print(torch.add(x, y))

# add and set result to other tensor
result = torch.empty(5, 3)
torch.add(x, y, out=result)
print(result)

# add x to y, overwriting y
y.add_(x)
print(y)

# all rows, first column
print(x[:, 1])

x = torch.randn(4, 4)
y = x.view(16)
z = x.view(-1, 8)  # the size -1 is inferred from other dimensions
print(x.size(), y.size(), z.size())

# tensor with one element, get value as number
x = torch.randn(1)
print(x)
print(x.item())

if torch.cuda.is_available():
    device = torch.device("cuda")
    y = torch.ones_like(x, device=device)  # tensor on GPU
    x = x.to(device)
    z = x + y
    print(z)
    print(z.to("cpu", torch.double))
