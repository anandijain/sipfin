using Plots, DataFrames, CSV, Glob, Dates
include(abspath("/home/sippycups/sipfin/jl/SipFin/src/SipFin.jl"))
using .SipFin

fns = glob("**.csv", readdir(abspath("/home/sippycups/D/nasdaq_o2/rt"), join = true))
df = vcat(SipFin.parse_rt.(CSV.read.(fns[end-50:end]))...)

function gen_anims(rt_df::DataFrame; size=(1200,1200))::DataFrame
    # prob way too slow
    by(rt_df, :symbol, p = (:t, :x, :v) => x->gen_anim(x.t, x,x, x.v))
end

function gen_anim(t, x, v::AbstractArray)::Animation
    plt = plot3d(
        1,
        xlim = (minimum(t), maximum(t)),
        ylim = (minimum(x), maximum(x)),
        zlim = (minimum(v), maximum(v)),
        title =  "time, price, volume",
        marker = 2,
        size=(1200, 1200)
    
    )

    anim = Animation(
    )
    for i=1:size(df,1)
        x, y, z = t[i], x[i], v[i]
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

