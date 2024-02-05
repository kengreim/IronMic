using Microsoft.AspNetCore.Authentication;
using OAuth.Vatsim;

namespace Microsoft.Extensions.DependencyInjection;

public static class VatsimAuthenticationExtensions
{
    /// <summary>
    /// Adds <see cref="VatsimAuthenticationHandler"/> to the specified
    /// <see cref="AuthenticationBuilder"/>, which enables Vatsim authentication capabilities.
    /// </summary>
    /// <param name="builder">The authentication builder.</param>
    /// <returns>A reference to this instance after the operation has completed.</returns>
    public static AuthenticationBuilder AddVatsim(this AuthenticationBuilder builder)
    {
        return builder.AddVatsim(VatsimAuthenticationDefaults.AuthenticationScheme, _ => { });
    }

    /// <summary>
    /// Adds <see cref="VatsimAuthenticationHandler"/> to the specified
    /// <see cref="AuthenticationBuilder"/>, which enables Vatsim authentication capabilities.
    /// </summary>
    /// <param name="builder">The authentication builder.</param>
    /// <param name="configuration">The delegate used to configure the OpenID 2.0 options.</param>
    /// <returns>A reference to this instance after the operation has completed.</returns>
    public static AuthenticationBuilder AddVatsim(
        this AuthenticationBuilder builder,
        Action<VatsimAuthenticationOptions> configuration)
    {
        return builder.AddVatsim(VatsimAuthenticationDefaults.AuthenticationScheme, configuration);
    }

    /// <summary>
    /// Adds <see cref="VatsimAuthenticationHandler"/> to the specified
    /// <see cref="AuthenticationBuilder"/>, which enables Vatsim authentication capabilities.
    /// </summary>
    /// <param name="builder">The authentication builder.</param>
    /// <param name="scheme">The authentication scheme associated with this instance.</param>
    /// <param name="configuration">The delegate used to configure the Vatsim options.</param>
    /// <returns>The <see cref="AuthenticationBuilder"/>.</returns>
    public static AuthenticationBuilder AddVatsim(
        this AuthenticationBuilder builder,
        string scheme,
        Action<VatsimAuthenticationOptions> configuration)
    {
        return builder.AddVatsim(scheme, VatsimAuthenticationDefaults.DisplayName, configuration);
    }

    /// <summary>
    /// Adds <see cref="VatsimAuthenticationHandler"/> to the specified
    /// <see cref="AuthenticationBuilder"/>, which enables Vatsim authentication capabilities.
    /// </summary>
    /// <param name="builder">The authentication builder.</param>
    /// <param name="scheme">The authentication scheme associated with this instance.</param>
    /// <param name="caption">The optional display name associated with this instance.</param>
    /// <param name="configuration">The delegate used to configure the Vatsim options.</param>
    /// <returns>The <see cref="AuthenticationBuilder"/>.</returns>
    public static AuthenticationBuilder AddVatsim(
        this AuthenticationBuilder builder,
        string scheme,
        string caption,
        Action<VatsimAuthenticationOptions> configuration)
    {
        return builder.AddOAuth<VatsimAuthenticationOptions, VatsimAuthenticationHandler>(scheme, caption, configuration);
    }
}