using Plots, DataFrames, CSV, Glob, Dates
include(abspath("/home/sippycups/sipfin/jl/SipFin/src/SipFin.jl"))
using .SipFin

fns = glob("**.csv", readdir(abspath("/home/sippycups/D/nasdaq_o2/rt"), join = true))
df = vcat(SipFin.parse_rt.(CSV.read.(fns[end-50:end]))...)




# @userplot TxvAnim
# @recipe function f(p::TxvAnim)
#     x, y, z, i = p.args
#     n = length(x)
#     inds = circshift(1:n, 1 - i)
#     linewidth --> range(0, 10, length = n)
#     seriesalpha --> range(0, 1, length = n)
#     aspect_ratio --> 1
#     label --> false
#     x[inds], y[inds], z[inds]
# end

