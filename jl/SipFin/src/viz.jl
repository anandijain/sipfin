using Plots, DataFrames, CSV, Glob, Dates
include(abspath("/home/sippycups/sipfin/jl/SipFin/src/SipFin.jl"))
using .SipFin

fns = glob("**.csv", readdir(abspath("/home/sippycups/D/nasdaq_o2/rt"), join = true))
df = vcat(SipFin.parse_rt.(CSV.read.(fns[end-50:end]))...)

function gen_anims(rt_df::DataFrame)::DataFrame
    for df in groupby(rt_df)
        

    end
end

function gen_anim(df::AbstractDataFrame)::Animation
    plt = plot3d(
        1,
        xlim = (minimum(df.t), maximum(df.t)),
        ylim = (minimum(df.x), maximum(df.x)),
        zlim = (minimum(df.v), maximum(df.v)),
        title = "$(df.symbol[1]): time, price, volume",
        marker = 2,
        size=(1200, 1200)
    
    )

    anim = Animation(
    )
    for i=1:size(df,1)
        x, y, z = df.t[i], df.x[i], df.v[i]
        push!(plt, (x, y, z))
        frame(anim)
    end
    anim
end


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

