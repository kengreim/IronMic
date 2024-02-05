using System.Security.Claims;
using System.Text.Json;
using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Authentication.OAuth;

namespace OAuth.Vatsim;

/// <summary>
/// Defines a set of options used by <see cref="VatsimAuthenticationHandler"/>.
/// </summary>
public class VatsimAuthenticationOptions : OAuthOptions
{
    public VatsimAuthenticationOptions()
    {
        ClaimsIssuer = VatsimAuthenticationDefaults.Issuer;

        CallbackPath = VatsimAuthenticationDefaults.CallbackPath;

        Server = VatsimAuthenticationServer.Development;

        ClaimActions.MapJsonSubKey(ClaimTypes.NameIdentifier, "data", "cid");
        ClaimActions.MapCustomJson(ClaimTypes.Name, GetFullName);
        ClaimActions.MapCustomJson(ClaimTypes.Email, GetEmail);

        Scope.Add(VatsimAuthenticationDefaults.Scopes.FullName);
        Scope.Add(VatsimAuthenticationDefaults.Scopes.VatsimDetails);
        Scope.Add(VatsimAuthenticationDefaults.Scopes.Email);
        Scope.Add(VatsimAuthenticationDefaults.Scopes.Country);
    }

    public VatsimAuthenticationServer Server
    {
        set
        {
            switch (value)
            {
                case VatsimAuthenticationServer.Production:
                    AuthorizationEndpoint = VatsimAuthenticationDefaults.Production.AuthorizationEndpoint;
                    TokenEndpoint = VatsimAuthenticationDefaults.Production.TokenEndpoint;
                    UserInformationEndpoint = VatsimAuthenticationDefaults.Production.UserInformationEndpoint;
                    break;
                
                case VatsimAuthenticationServer.Development:
                    AuthorizationEndpoint = VatsimAuthenticationDefaults.Development.AuthorizationEndpoint;
                    TokenEndpoint = VatsimAuthenticationDefaults.Development.TokenEndpoint;
                    UserInformationEndpoint = VatsimAuthenticationDefaults.Development.UserInformationEndpoint;
                    break;
                
                default:
                    throw new ArgumentOutOfRangeException(nameof(value), value, $"Server '{value}' is unsupported.");
            }
        }
    }

    private static string? GetFullName(JsonElement root) =>
        root.GetProperty("data").GetProperty("personal").GetString("name_full");

    private static string? GetEmail(JsonElement root) =>
        root.GetProperty("data").GetProperty("personal").GetString("email");
}
