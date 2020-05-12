## todo function that composes 
function corr_5(dfs)
    m = join(rand(dfs, 5)..., on = :t)
    mat = Matrix(m)[:, 2:5:end]
    labs = names(m)[2:5:end]    
    corrplot(mat, label = labs)
end

function sa_yf()
    # we're assuming we just ran finox w cargo run,
    now = Dates.today()
    yr = string(year(now))
    month = string(Dates.month(now))
    day = string(Dates.day(now))
    # fns = Glob.glob("*:US_$(yr)_$(month)_$(day).csv", "./data/")
    sa = CSV.read("./ref_data/sa.csv")
    sa_slugs = uppercase.(unique(dropmissing(sa, :slug).slug))
    # reg = r"data\/(.*):"
    # matches = match.(reg, fns)
    # yf_slugs = map(x -> x.captures[1], matches)
    yf_slugs = readlines("./ref_data/tickers.txt")
    intersect_ticks = intersect(sa_slugs, yf_slugs)
    println(intersect_ticks)
    str_arr_to_txt("./ref_data/intersect_sa_yf.txt", intersect_ticks)
end


function sec13f_fix()
    for fn in Glob.glob("./ref_data/rentec/*.csv")
        header = Symbol.(["nameOfIssuer",
        "titleOfClass",
        "cusip",
        "value",
        "sshPrnamt",
        "sshPrnamtType",
        "investmentDiscretion",
        "otherManager",
        "Sole",
        "Shared",
        "None"])
        df = DataFrame(permutedims(Matrix(CSV.read(fn, header = false))), header)
        print(df)
        CSV.write(fn, df)
    end
end