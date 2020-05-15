using Plots, DataFrames, CSV, Glob, Dates
include("./SipFin/src/SipFin.jl")
using .SipFin

fns = readdir(abspath("/home/sippycups/D/nasdaq_o2/rt"), join = true)

df = vcat(SipFin.parse_rt.(CSV.read.(fns[1:50]))...)
to_plot = copy(dropmissing(df[df.symbol .== "amzn", :]))
to_plot.t = today() .+ to_plot.t
to_plot.t = datetime2unix.(to_plot.t)
# to_plot[:, 2:end] = hcat(SipFin.norm_arr.(eachcol(to_plot[:, 2:end]))...)
sort!(to_plot, :t)

# initialize a 3D plot with 1 empty series
plt = plot3d(
    1,
    xlim = (minimum(to_plot.t), maximum(to_plot.t)),
    ylim = (minimum(to_plot.x), maximum(to_plot.x)),
    zlim = (minimum(to_plot.v), maximum(to_plot.v)),
    title = "t, x, v",
    marker = 2,
)

n = size(to_plot, 1)
j = range(0, stop = 2π, length = n)
x, y, z = eachcol(to_plot[:, [:t, :x, :v]])
@gif for i = 1:n
    push!(plt, x[i], y[i], z[i])
end 
