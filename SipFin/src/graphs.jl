using DataFrames, LightGraphs, GraphPlot, SimpleWeightedGraphs, MetaGraphs, GraphRecipes

function cor_df_to_edge_weights(df::AbstractDataFrame, ret_df::Bool = true)::Union{AbstractDataFrame,Matrix}
    @assert size(df, 1) == size(df, 2)
    dim = size(df, 1)
    if ret_df
        DataFrame(ones(dim, dim) - Matrix(df), names(df))
    else
        ones(dim, dim) - Matrix(df)
    end
end

function edges_df_to_graph(df::AbstractDataFrame, threshold = 0.6)::SimpleWeightedGraph
    @assert size(df, 1) == size(df, 2) # assert square
    dim = size(df, 1)
    g = SimpleWeightedGraph(dim)
    # zero weights do nothing thankfully
    for i in 1:dim
        for j in 1:dim
            val = df[i, j]
            val < threshold ? add_edge!(g, i, j, val) : continue
        end
    end
    g
end

function edges_df_to_meta_graph(df::AbstractDataFrame)::MetaGraph
    @assert size(df, 1) == size(df, 2) # assert square
    dim = size(df, 1)
    g = MetaGraph(dim)
    for i in 1:dim
        for j in 1:dim
            val = df[i, j]
            add_edge!(g, i, j)
            set_prop!(g, Edge(i, j), :weight, val)
        end
    end
    g
end


df_to_cor_graph(df::AbstractDataFrame, threshold::Float64 = 0.8) = edges_df_to_graph(cor_df_to_edge_weights(cor_df(df)), threshold)
df_to_meta_cor_graph(df::AbstractDataFrame) = edges_df_to_meta_graph(cor_df_to_edge_weights(cor_df(df)))


# thanks seth
to_prune(g, n) = findall(LightGraphs.weights(g) .> n)
to_prune_nodes(g, n::Integer) = findall((g) .> n)
