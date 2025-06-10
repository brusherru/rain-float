// Generate bindings for the Float contract
alloy_sol_types::sol! {
    #![sol(all_derives = true)]
    DecimalFloat,
    "../vendor/rain.math.float/out/DecimalFloat.sol/DecimalFloat.json"
}