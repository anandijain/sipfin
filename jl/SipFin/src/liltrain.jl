using Flux, CSV, DataFrames, Glob
include("./utils.jl")
using .Utils
using Flux: @epochs

# dfs = get_dfs("./data/**HIST*.csv")
df = CSV.read("./big_bois.csv")
ddf = Utils.diff(df)[:, 2:end]
mat = Matrix{Float32}(ddf)
dim = size(mat, 2)

data = []

for i in 1:size(mat, 1)-1
    x = mat[i, :]
    y= mat[i+1, :]

    push!(data, [x, y])
end


m = Chain(
    Dense(dim, 100, relu), 
    Dense(100, 100, relu), 
    # Dense(100, 100, relu), 
    Dense(100, dim)
    )

loss(x, y) = Flux.mse(m(x), y)
ps = Flux.params(m)
opt = ADAM()
# X = mat[1:end-1, :]
# Y = mat[2:end, :]
# X, Y = df[1:end, :], df[2]


@epochs 50 Flux.train!(loss, ps, data, opt)

for i in 1:size(mat, 1) -1 
    println("$i, $(loss(data[i]...))")
    x, y, ŷ = data[i][1], data[i][2], m(data[i][1])
    ls = (ŷ .- y) .^ 2
    println("$(DataFrame(hcat(x, y, ŷ, ls)))")
end


# function trainloop!(loss, ps, data, opt)
#     ps = Flux.params(ps)
#     for (i, d) in enumerate(data)
#         gs = gradient(ps) do
#         training_loss = loss(d...)
#         if i % 8000 == 0
#             println("$i: $training_loss")
#         end
#     # Insert what ever code you want here that needs Training loss, e.g. logging
#         return training_loss
#     end
#     # insert what ever code you want here that needs gradient
#     # E.g. logging with TensorBoardLogger.jl as histogram so you can see if it is becoming huge
#     Flux.update!(opt, ps, gs)
#     # Here you might like to check validation set accuracy, and break out to do early stopping
# end
# end

# @epochs 100 trainloop!(loss, ps, data, opt)
