using Plots, CSV, DataFrames, Glob, Dates
using Statistics, StatsBase

fns = Glob.glob("USD*", "./data/")
dfs = CSV.read.(fns)

# df = dfs[1]
# dtfmt = DateFormat("Y-m-dTH:M:SZ")
# for df in dfs
#     df.date_time = Date.(df.date_time, dtfmt)
# end
norm_arr(a::AbstractArray) = (a .- mean(a)) ./ std(a)
arrs = [df[:, 2] for df in dfs][1:end-3]
arrs = norm_arr.(arrs)
# arrs = [arr .- arr[1] for arr in arrs]

curs = DataFrame(names.(dfs))[2, :]
curs = string.(copy(Array(curs)))[1:end-3]

x = plot(arrs[1], labels=curs[1], size=(1600,1600))
for i in 2:length(arrs)
    plot!(x, arrs[i], label=curs[i])
end
display(x)
# for i in 1:length(arrs)
#     plot!(arrs[i], label=curs[i])
# end