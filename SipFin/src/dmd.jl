using DataDrivenDiffEq, Flux 
using CSV, DataFrames, Plots, Dates, LinearAlgebra, Statistics

fns = readdir("/home/sippycups/repos/sipfin/data/yf/", join=true)
dfs = (dropmissing! ∘ DataFrame! ∘ CSV.File).(fns)
dfd = Dict(zip((last ∘ split).(fns, "/"), dfs))
cols = names(dfs[1])
regexs = map(x-> Regex("^$(x)_\\d"), cols)
biggos = filter(x -> nrow(x) > 9000, dfs)

length(biggos)
j = innerjoin(biggos[1:100]..., on=:t, makeunique=true)
yos = map(x->Matrix(j[:, x])', regexs[3:end])
symbs = Vector(j[1, r"^symbol_\d"])
dmds = DMD.(yos)

ops = [x.operator for x in dmds]
preds = [ops[i] .* eachrow(yos[i]) for i in 1:length(ops)]

for i in 1:size(ops, 1)
    for j in 1:size(ops[i], 1)
        loss = Flux.mse(preds[i][j], yos[i][j, :])
        println("$(symbs[j]) $(cols[i+2]): $(loss)")
    end
end

Fs = map(x->svd(x'), yos)
vals = [x.S for x in Fs]
p = plot()
display(plot!(p, vals[1:end-1]))
rank.(yos, rtol=1e-3)


nums = j[:, Not(r"^symbol")]
nums = nums[:, 2:end]
vars = var.(eachcol(X[2:end, :] - X[1:end-1, :]))
V = (X[end, :] - X[1, :]) ./ vars
vals = Dict(zip(symbs, V))
DataFrame(vals)